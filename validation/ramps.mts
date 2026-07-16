import Color from "colorjs.io";
const toHex = (L: number, C: number, H: number) =>
  new Color("oklch", [L, C, H]).to("srgb").toGamut({ method: "css" }).toString({ format: "hex" });

// Sequential: Meridian blue, hue 240 (Maritime-anchored), 13 steps 100→700.
// L descends 0.92→0.32; chroma arcs low→peak(≈0.13 mid)→0.10.
const STEPS = [100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600, 650, 700];
const seq = STEPS.map((s, i) => {
  const t = i / (STEPS.length - 1);
  const L = 0.905 - t * (0.905 - 0.315);
  const C = 0.045 + 0.085 * Math.sin(Math.PI * Math.min(1, t * 1.15)); // peak just past mid
  return [s, toHex(L, C, 240)] as const;
});
console.log("SEQUENTIAL blue-240:");
console.log(seq.map(([s, h]) => `${s}:${h}`).join(" "));

// Diverging arms: 5 steps per arm toward each pole + neutral midpoint.
// Candidate A: maritime blue (240) ↔ burnt orange (55)
// Candidate B: maritime blue (240) ↔ brick red (25)
const arm = (H: number) =>
  [0.82, 0.72, 0.62, 0.52, 0.42].map((L, i) => toHex(L, 0.05 + 0.025 * i, H));
console.log("\nDIVERGING A (blue<-mid->orange):");
console.log([...arm(240).reverse()].join(",") + " | mid #f4f3f2/#2a2928 | " + arm(55).join(","));
console.log("DIVERGING B (blue<-mid->red):");
console.log([...arm(240).reverse()].join(",") + " | mid #f4f3f2/#2a2928 | " + arm(25).join(","));

// Status (fixed across modes; distinct from categorical slots):
console.log("\nSTATUS:");
console.log("good     " + "#218a45");
console.log("warning  " + "#da950b  (black text on badges — bright-scale caveat)");
console.log("serious  " + toHex(0.66, 0.15, 45));
console.log("critical " + "#c9302d");

// null_ink: warm gray step 5 — reads 'no data', can't impersonate a scheme value
console.log("\nNULL_INK light #dcdad8  dark #32302f");
