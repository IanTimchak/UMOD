const invoke = window.__TAURI__.core.invoke;
const listen = window.__TAURI__.event.listen;

const box = document.getElementById("selection-box");
const topS = document.getElementById("shade-top");
const botS = document.getElementById("shade-bottom");
const leftS = document.getElementById("shade-left");
const rightS = document.getElementById("shade-right");

//show overlay
invoke("rs_ready");

// -------- input → backend --------

let lastX = 0;
let lastY = 0;
let needsSend = false;

window.addEventListener("mousemove", (e) => {
  lastX = e.clientX;
  lastY = e.clientY;
  needsSend = true;
});

window.addEventListener("mousedown", (e) => {
  if (e.button === 0) {
    invoke("rs_mousedown", { button: "left" });
  }
});

window.addEventListener("mouseup", (e) => {
  if (e.button === 0) {
    invoke("rs_mouseup", { button: "left" });
  }
});

//establish size
invoke("rs_set_window_size", {
  width: window.innerWidth,
  height: window.innerHeight,
});

window.addEventListener("keydown", (e) => {
  if (e.key === "Enter") {
    invoke("rs_key_enter");
  } else if (e.key === "Escape") {
    invoke("rs_key_escape");
  }
});

window.addEventListener("resize", () => {
  invoke("rs_set_window_size", {
    width: window.innerWidth,
    height: window.innerHeight,
  });
});

// coalesced cursor updates
function cursorLoop() {
  if (needsSend) {
    needsSend = false;
    invoke("rs_cursor", { x: lastX, y: lastY });
  }
  requestAnimationFrame(cursorLoop);
}
cursorLoop();

// -------- state → render --------
//
// rs_get_state must return:
// { phase: "Idle" | "Drawing" | "Confirmed" | "Moving" | "Capturing",
//   bounds: { x, y, w, h } | null,
//   screen: { w, h } }

function setShadeRect(el, x, y, w, h) {
  el.style.left = x + "px";
  el.style.top = y + "px";
  el.style.width = w + "px";
  el.style.height = h + "px";
  el.style.display = w > 0 && h > 0 ? "block" : "none";
}

function applyState(state) {
  if (!state) {
    box.style.display = "none";
    // full-screen dim
    setShadeRect(topS, 0, 0, window.innerWidth, window.innerHeight);
    botS.style.display = leftS.style.display = rightS.style.display = "none";
    return;
  }

  const { phase, bounds, screen } = state;

  const scrW = screen?.w ?? window.innerWidth;
  const scrH = screen?.h ?? window.innerHeight;

  if (!bounds) {
    box.style.display = "none";
    // full-screen dim
    setShadeRect(topS, 0, 0, scrW, scrH);
    botS.style.display = leftS.style.display = rightS.style.display = "none";
    return;
  }

  const { x, y, w, h } = bounds;
  console.log(bounds);

  if (phase === "Capturing") {
    // no dim, no box – clean screenshot
    box.style.display = "none";
    topS.style.display =
      botS.style.display =
      leftS.style.display =
      rightS.style.display =
        "none";
    return;
  }

  // position selection box
  box.style.display = "block";
  box.style.transform = `translate(${x}px, ${y}px)`;
  box.style.width = w + "px";
  box.style.height = h + "px";

  // phase-specific styling
  box.classList.remove("drawing", "marching");
  if (phase === "Drawing") {
    box.classList.add("drawing");
  } else if (phase === "Confirmed" || phase === "Moving") {
    box.classList.add("marching");
  }

  // build four dim rectangles around the selection
  // top
  setShadeRect(topS, 0, 0, scrW, y);
  // bottom
  setShadeRect(botS, 0, y + h, scrW, Math.max(scrH - (y + h), 0));
  // left
  setShadeRect(leftS, 0, y, x, h);
  // right
  setShadeRect(rightS, x + w, y, Math.max(scrW - (x + w), 0), h);
}

let capture_flag = false;

// backend tells us when something changed
listen("rs-update", async () => {
  if (!capture_flag) {
    try {
      const state = await invoke("rs_get_state");
      applyState(state);
    } catch (err) {
      console.error("rs-update / rs_get_state error:", err);
    }
  }
});

listen("rs-begin-capture", async () => {
  capture_flag = true;
  try {
    const state = await invoke("rs_get_state");
    applyState(state);
    // After UI becomes transparent, request screenshot
    setTimeout(() => {
      invoke("rs_do_capture");
    }, 5); // 1–5 ms gives browser one frame to repaint
  } catch (err) {
    console.error("rs-update / rs_get_state error:", err);
  }
});
