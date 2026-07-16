import Color from "colorjs.io";

// ── colour maths (mirrors validate_palette.js exactly) ──
const s2lin = (c: number) => (c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4);
const hex2lin = (h: string) => {
  h = h.replace(/^#/, "");
  return [0, 2, 4].map((i) => s2lin(parseInt(h.slice(i, i + 2), 16) / 255));
};
const MACHADO: Record<string, number[][]> = {
  protan: [[0.152286, 1.052583, -0.204868], [0.114503, 0.786281, 0.099216], [-0.003882, -0.048116, 1.051998]],
  deutan: [[0.367322, 0.860646, -0.227968], [0.280085, 0.672501, 0.047413], [-0.01182, 0.04294, 0.968881]],
};
const oklabFromLin = ([r, g, b]: number[]) => {
  const l = Math.cbrt(0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b);
  const m = Math.cbrt(0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b);
  const s = Math.cbrt(0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b);
  return [
    0.2104542553 * l + 0.793617785 * m - 0.0040720468 * s,
    1.9779984951 * l - 2.428592205 * m + 0.4505937099 * s,
    0.0259040371 * l + 0.7827717662 * m - 0.808675766 * s,
  ];
};
const sim = (lin: number[], M: number[][]) =>
  M.map((row) => Math.max(0, Math.min(1, row[0] * lin[0] + row[1] * lin[1] + row[2] * lin[2])));
const dE = (h1: string, h2: string, kind?: string) => {
  const a = oklabFromLin(kind ? sim(hex2lin(h1), MACHADO[kind]) : hex2lin(h1));
  const b = oklabFromLin(kind ? sim(hex2lin(h2), MACHADO[kind]) : hex2lin(h2));
  return 100 * Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
};
const cvd = (h1: string, h2: string) => Math.min(dE(h1, h2, "protan"), dE(h1, h2, "deutan"));
const relLum = (h: string) => {
  const [r, g, b] = hex2lin(h);
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
};
const contrast = (a: string, b: string) => {
  const [hi, lo] = [relLum(a), relLum(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
};

// OKLCH spec → gamut-mapped hex
const toHex = (L: number, C: number, H: number) =>
  new Color("oklch", [L, C, H]).to("srgb").toGamut({ method: "css" }).toString({ format: "hex" });

// ── Harbour candidate: [name, hue, lightL, lightC, darkL, darkC] ──
// Slot values are DESIGN INPUTS — iterate on the printed metrics.
type Spec = [string, number, number, number, number, number];
const HARBOUR: Spec[] = [
  ["blue",   240, 0.58, 0.140, 0.60, 0.135],
  ["orange",  55, 0.68, 0.150, 0.57, 0.140],
  ["teal",   172, 0.665, 0.115, 0.665, 0.115],
  ["gold",    88, 0.73, 0.140, 0.67, 0.130],
  ["plum",   335, 0.63, 0.150, 0.61, 0.140],
  ["green",  145, 0.60, 0.130, 0.60, 0.130],
  ["violet", 290, 0.52, 0.150, 0.58, 0.140],
  ["red",     25, 0.55, 0.170, 0.58, 0.160],
];
const SURFACE = { light: "#fcfcfb", dark: "#161413" };
const BAND = { light: [0.43, 0.77], dark: [0.48, 0.67] };

const build = (mode: "light" | "dark") => {
  const hexes = HARBOUR.map(([nm, H, lL, lC, dL, dC]) =>
    mode === "light" ? [nm, toHex(lL, lC, H)] : [nm, toHex(dL, dC, H)]
  ) as [string, string][];
  const k = hexes.length;
  const cvdM: number[][] = [], normM: number[][] = [];
  for (let i = 0; i < k; i++) {
    cvdM[i] = []; normM[i] = [];
    for (let j = 0; j < k; j++) {
      cvdM[i][j] = i === j ? Infinity : cvd(hexes[i][1], hexes[j][1]);
      normM[i][j] = i === j ? Infinity : dE(hexes[i][1], hexes[j][1]);
    }
  }
  return { hexes, cvdM, normM };
};
const L = build("light"), D = build("dark");
const n = L.hexes.length;

for (const [mode, B] of [["light", L], ["dark", D]] as const) {
  console.log(`=== HARBOUR ${mode} on ${SURFACE[mode]} ===`);
  for (const [nm, h] of B.hexes) {
    const [l, a, b] = oklabFromLin(hex2lin(h));
    const C = Math.hypot(a, b);
    const inBand = l >= BAND[mode][0] && l <= BAND[mode][1];
    const ct = contrast(h, SURFACE[mode]);
    console.log(`${nm.padEnd(7)} ${h}  L${l.toFixed(3)}${inBand ? " " : "!"} C${C.toFixed(3)}${C >= 0.1 ? " " : "!"} contrast ${ct.toFixed(2)}${ct >= 3 ? "" : " (relief)"}`);
  }
}

// joint ordering search, slot 1 pinned to blue
const rest = [...Array(n).keys()].slice(1);
type Cand = { order: number[]; adjCvd: number; adjNorm: number; f4Cvd: number; f4Norm: number };
const cands: Cand[] = [];
const evalOrder = (order: number[]): Cand => {
  let adjCvd = Infinity, adjNorm = Infinity, f4Cvd = Infinity, f4Norm = Infinity;
  for (const B of [L, D]) {
    for (let i = 0; i < n - 1; i++) {
      adjCvd = Math.min(adjCvd, B.cvdM[order[i]][order[i + 1]]);
      adjNorm = Math.min(adjNorm, B.normM[order[i]][order[i + 1]]);
    }
    for (let i = 0; i < 4; i++)
      for (let j = i + 1; j < 4; j++) {
        f4Cvd = Math.min(f4Cvd, B.cvdM[order[i]][order[j]]);
        f4Norm = Math.min(f4Norm, B.normM[order[i]][order[j]]);
      }
  }
  return { order, adjCvd, adjNorm, f4Cvd, f4Norm };
};
const permute = (arr: number[], cur: number[]) => {
  if (!arr.length) {
    const c = evalOrder([0, ...cur]);
    if (c.adjNorm >= 15 && c.f4Norm >= 15 && c.f4Cvd >= 6) cands.push(c);
    return;
  }
  for (let i = 0; i < arr.length; i++)
    permute([...arr.slice(0, i), ...arr.slice(i + 1)], [...cur, arr[i]]);
};
permute(rest, []);
cands.sort((a, b) => b.adjCvd - a.adjCvd || b.f4Cvd - a.f4Cvd || b.adjNorm - a.adjNorm);
console.log(`\njoint orderings w/ adjNorm>=15, f4Norm>=15, f4CVD>=6 in BOTH modes: ${cands.length}/5040; top 3:`);
for (const c of cands.slice(0, 3))
  console.log(`  ${c.order.map((i) => L.hexes[i][0]).join(" > ")}\n    minAdjCVD ${c.adjCvd.toFixed(1)}  minAdjNorm ${c.adjNorm.toFixed(1)}  first4: allPairsCVD ${c.f4Cvd.toFixed(1)} allPairsNorm ${c.f4Norm.toFixed(1)}`);
if (cands.length) {
  const w = cands[0];
  console.log("\nWINNER light: " + w.order.map((i) => L.hexes[i][1]).join(","));
  console.log("WINNER dark:  " + w.order.map((i) => D.hexes[i][1]).join(","));
}

// diagnostics: joint (min-over-mode) pairwise CVD matrix + best blue-quads
const names = L.hexes.map(([nm]) => nm);
console.log("\njoint min CVD matrix (light,dark min):");
console.log("        " + names.map((x) => x.slice(0, 6).padStart(7)).join(""));
for (let i = 0; i < n; i++) {
  let row = names[i].padEnd(8);
  for (let j = 0; j < n; j++)
    row += (i === j ? "·" : Math.min(L.cvdM[i][j], D.cvdM[i][j]).toFixed(1)).padStart(7);
  console.log(row);
}
const quads: { q: number[]; mc: number; mn: number }[] = [];
for (let a = 1; a < n; a++) for (let b = a + 1; b < n; b++) for (let c = b + 1; c < n; c++) {
  const q = [0, a, b, c];
  let mc = Infinity, mn = Infinity;
  for (const B of [L, D])
    for (let i = 0; i < 4; i++) for (let j = i + 1; j < 4; j++) {
      mc = Math.min(mc, B.cvdM[q[i]][q[j]]); mn = Math.min(mn, B.normM[q[i]][q[j]]);
    }
  quads.push({ q, mc, mn });
}
quads.sort((x, y) => y.mc - x.mc);
console.log("\nbest blue-anchored quads (minPairCVD, minPairNorm):");
for (const { q, mc, mn } of quads.slice(0, 5))
  console.log(`  ${q.map((i) => names[i]).join("+")}  CVD ${mc.toFixed(1)}  norm ${mn.toFixed(1)}`);
