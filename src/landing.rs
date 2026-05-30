//! Animated galaxy landing page (route `/`).
//!
//! The favicon (a DNA double-helix logo) sits at the centre over a rotating
//! galaxy/nebula starfield, wrapped by an animated CSS DNA helix in the
//! favicon's own cyan/red strand colours. The whole screen is "click to
//! enter": visuals animate on load, and the first click starts a synthesized
//! cosmic sound (Web Audio — browsers block autoplay without a gesture),
//! plays a warp-zoom transition, then routes into the app.

use dioxus::prelude::*;

use crate::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");

/// Number of base-pair rungs in the CSS DNA helix.
const RUNGS: i32 = 24;

/// Self-contained Web Audio "cosmic startup" sound: a low drone pad, a
/// shimmering tremolo, a rising minor-pentatonic arpeggio, and a noise "warp"
/// whoosh for the climax. Fire-and-forget; closes its context after ~6.5s.
const LANDING_SOUND: &str = r#"
(function(){
  // Tell Rust to navigate after the warp transition — outside the audio
  // try/catch so it fires even if Web Audio is unavailable.
  setTimeout(function(){ try{ dioxus.send(true); }catch(e){} }, 1700);
  try{
    var AC = window.AudioContext || window.webkitAudioContext;
    if(!AC) return;
    var ctx = new AC();
    if(ctx.state === 'suspended'){ ctx.resume(); }
    var now = ctx.currentTime;
    var master = ctx.createGain();
    master.gain.setValueAtTime(0.0001, now);
    master.gain.exponentialRampToValueAtTime(0.5, now + 1.0);
    master.gain.setValueAtTime(0.5, now + 3.5);
    master.gain.exponentialRampToValueAtTime(0.0001, now + 6.0);
    master.connect(ctx.destination);

    // Drone pad
    [110, 110.6, 165].forEach(function(f, i){
      var o = ctx.createOscillator();
      o.type = i < 2 ? 'sine' : 'triangle';
      o.frequency.value = f;
      var g = ctx.createGain();
      g.gain.value = i < 2 ? 0.18 : 0.08;
      o.connect(g).connect(master);
      o.start(now); o.stop(now + 6);
    });

    // Shimmer with tremolo
    var sh = ctx.createOscillator(); sh.type = 'sine'; sh.frequency.value = 880;
    var shg = ctx.createGain(); shg.gain.value = 0.035;
    var lfo = ctx.createOscillator(); lfo.frequency.value = 6;
    var lfog = ctx.createGain(); lfog.gain.value = 0.03;
    lfo.connect(lfog).connect(shg.gain);
    sh.connect(shg).connect(master);
    sh.start(now); sh.stop(now + 6); lfo.start(now); lfo.stop(now + 6);

    // Rising arpeggio (A minor pentatonic)
    [440, 523.25, 587.33, 659.25, 783.99, 880].forEach(function(f, i){
      var t = now + 0.15 + i * 0.16;
      var o = ctx.createOscillator(); o.type = 'triangle'; o.frequency.value = f;
      var g = ctx.createGain();
      g.gain.setValueAtTime(0.0001, t);
      g.gain.exponentialRampToValueAtTime(0.25, t + 0.02);
      g.gain.exponentialRampToValueAtTime(0.0001, t + 0.5);
      o.connect(g).connect(master); o.start(t); o.stop(t + 0.55);
    });

    // Warp whoosh climax (filtered noise sweep)
    var dur = 1.6;
    var buf = ctx.createBuffer(1, Math.floor(ctx.sampleRate * dur), ctx.sampleRate);
    var d = buf.getChannelData(0);
    for(var i = 0; i < d.length; i++){ d[i] = Math.random() * 2 - 1; }
    var noise = ctx.createBufferSource(); noise.buffer = buf;
    var bp = ctx.createBiquadFilter(); bp.type = 'bandpass'; bp.Q.value = 1.2;
    bp.frequency.setValueAtTime(200, now + 0.2);
    bp.frequency.exponentialRampToValueAtTime(6000, now + 0.2 + dur);
    var ng = ctx.createGain();
    ng.gain.setValueAtTime(0.0001, now + 0.2);
    ng.gain.exponentialRampToValueAtTime(0.25, now + 0.6);
    ng.gain.exponentialRampToValueAtTime(0.0001, now + 0.2 + dur);
    noise.connect(bp).connect(ng).connect(master);
    noise.start(now + 0.2); noise.stop(now + 0.2 + dur);

    setTimeout(function(){ try{ ctx.close(); }catch(e){} }, 6500);
  }catch(e){}
})();
"#;

#[component]
pub fn Landing() -> Element {
    let nav = use_navigator();
    let mut entering = use_signal(|| false);

    let begin = move |_| {
        if entering() {
            return;
        }
        entering.set(true);
        // Start the sound (allowed now that we're inside a user gesture). The
        // script also signals back via `dioxus.send` after the warp transition
        // so we can navigate into the app — no timer dependency needed.
        let mut eval = document::eval(LANDING_SOUND);
        spawn(async move {
            let _ = eval.recv::<bool>().await;
            nav.push(Route::Progress {});
        });
    };

    rsx! {
        div {
            class: if entering() { "landing entering" } else { "landing" },
            onclick: begin,

            div { class: "landing-nebula" }
            div { class: "landing-stars stars-near" }
            div { class: "landing-stars stars-mid" }
            div { class: "landing-stars stars-far" }

            div { class: "landing-stage",
                div { class: "landing-pulse" }
                div { class: "landing-pulse p2" }
                div { class: "landing-pulse p3" }

                div { class: "dna",
                    {(0..RUNGS).map(|i| {
                        let top = i * 16;
                        let delay = -(i as f64) * 0.16;
                        rsx! {
                            div {
                                key: "{i}",
                                class: "rung",
                                style: "top:{top}px;animation-delay:{delay:.2}s",
                                div { class: "node na" }
                                div { class: "node nb" }
                                div { class: "bar" }
                            }
                        }
                    })}
                }

                img { class: "landing-logo", src: FAVICON, alt: "Quant Roadmap" }
            }

            h1 { class: "landing-title", "Quant Roadmap" }
            div { class: "landing-sub", "decode the path · day by day" }
            button { class: "landing-enter", "▶  Enter  ·  sound on" }
        }
    }
}
