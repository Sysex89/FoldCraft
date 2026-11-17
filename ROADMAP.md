# ClothCraft roadmap

**Branch update (FC-0047; formerly branch-local FC-0040):** desktop-enabled entry points now default to the independent [resident WGSL finite AVBD / IPC-style pipeline](docs/wgsl-avbd-ipc.md). CPU-only builds retain CPU execution. Historical VBD/OGC descriptions and timings below describe the retained controls; `--hybrid-vbd` selects the old hybrid, while `--gpu-sim` selects the new default. This selection does not establish performance or full IPC/AVBD equivalence.

Build a useful garment authoring tool whose physics can evolve independently. Public Marvelous Designer manuals inform expected interactions; ClothCraft implements its own code, data model, UI and test fixtures. This is an independent implementation effort, not an affiliated product or a claim of formal clean-room certification.

Status: research prototype, November 2026. **Most of the product is still missing. A roadmap item is not a supported feature.** Read the [source/behavior ledger](docs/manual-study.md), [current authoring guide](docs/authoring.md) and [input/output capability matrix](docs/interoperability.md).

FC-0022 replaced the egui desktop UI with the native cssimpler workspace. Many editing tools that the egui UI exposed survive only as CPU commands; [native UI status](docs/native-ui-status.md) lists what `cloth-fit --view` actually wires up. Checklists in this file use three states:

- `[x]` reachable in the native editor (or, for headless items, in the CLI) and covered by tests.
- `[~]` implemented in the CPU model or the removed egui UI, but not reachable in the native editor. This is not a supported feature.
- `[ ]` not implemented.

The [15 October essential-tool review](docs/essential-authoring-tools.md) compares the current code with official CLO/Marvelous workflows. It proposes user-visible slices and acceptance examples: reliable documents, partial and multi-edge sewing, pins/freeze, arrangement, measurements and visibility, offsets/cuts, linked symmetry and grain. Manufacturing-specific release gates are separate. This is research and planning, not an implementation update.

## Current contribution queue

The [EXO comparison](docs/exo-feature-audit.md) and [rewrite backlog](docs/rewrite-backlog.md) retain upstream planning context. They are optional references. Concrete implementation tickets (`FC-xxxx`) are scheduled in the [software factory](docs/software-factory.md) queue (`implementation/queue.json`, `python3 scripts/factory.py list`); each ticket should serve one of the tracks below. A backlog entry or ticket is not a completed feature.

Updated 17 November 2026. The first goal is a coherent editing workflow around the existing solver, starting with the native editor parity in UX-00. The physics work needs reproducible evidence, not a new default engine selected on reputation. Contributors may work independently on these tracks; preserve document compatibility when changing shared schemas. [Task prompts and reporting templates](docs/contributor-kit.md) are available for people using coding assistants.

| ID | Remaining task | Starting point / dependencies | Acceptance |
|---|---|---|---|
| UX-00 | Restore the core save → sew → arrange → reload loop in the native editor | "Not ported" table in [native UI status](docs/native-ui-status.md); `atelier/css_runner.rs`; FC-0060…0065 cover the first fixes | Open, Save to the opened file, Save As, dirty state and recovery; 3D edge sewing and seam overlays; placement; partial-range sewing. The release-evidence T-shirt scenario below runs in `cloth-fit --view` without the CLI or JSON edits |
| UX-01 | Validate and refine the first-time sewing workflow | `atelier/css_runner.rs` (2D Sew tool); existing partial/chain model; 3D edge sewing not yet ported ([status](docs/native-ui-status.md)) | An unfamiliar user selects two intended boundaries, sees direction and unequal-length policy, confirms once, then selects/deletes/undoes the seam in either view |
| UX-02 | Consistent selection and editing across pieces | `atelier/css_runner.rs`, `authoring.rs`, `authoring/clipboard.rs` | Box/lasso, point/piece selection, group edits and Delete have explicit scope; text fields retain keyboard ownership; one edit is one undo entry; linked mirrors do not update twice |
| UX-03 | Clear inspection and viewport controls | `atelier/css_runner.rs`, `atelier/css_ui/` | A clean mesh view is reachable in one action; selection and sewing overlays have predictable visibility; Grab radius/strength are discoverable and do not change materials |
| UX-04 | Arrangement and measurements for real drafting | ARR-01, SIM-01, `Command::{Place, PlaceGroup}` (no native placement gizmo yet) | Body-aware placement, pin editing and useful measurements preserve material coordinates and report unsupported operations |
| PHYS-01 | Diagnose persistent motion on a settled benchmark | `vbd/audit.rs`, `motion.rs` for the VBD/OGC controls; `gpu_avbd/` (default) does not yet report a final residual; owner-provided final benchmark pending | Distinguish assembly motion, cold-restart history, contact-refresh discontinuity and final force residual; preserve velocities and record identical pull/hold/release controls |
| PHYS-02 | Continuous contact response without artificial sticking | `gpu_avbd/` (default; FC-0044 tangential-force fix), `ogc/` for the retained controls, existing rejected contact trials | Preserve separation checks and force coverage; pass rest, frictionless sliding and release; report energy/force continuity and full-step cost; keep unproven variants opt-in |
| PHYS-03 | Evaluate real alternative solver adapters | `solver_lab/`, solver challenge | Same rest mesh/materials/seams/controls; report quality before speed; do not expose failed lab candidates as editor modes |
| PHYS-04 | Rod-based garment dynamics (research track) | `cosserat.rs` (CPU Cosserat rods for authored rod geometry, FC-0051); FC-0052…0058 | Opt-in only. Rod topology compiles from real panels without changing rest geometry or legacy projects; on the PHYS-03 matched scenes it reports quality and cost against the default backend; it enters the native solver menu (FC-0054) only after passing those comparisons |
| PERF-01 | Reduce complete-step cost on dense garments | `gpu_avbd/` (default), with `gpu_vbd/` and `ogc/` as controls; baseline before optimization | Same mesh, dt, iterations and collision scope; P50/P95, simulated/wall time, GPU waits and CPU costs; independent collision and deformation checks |
| MAT-01 | Replace estimates with measured material fits | `fabrics.rs`, `energy.rs`; reusable source data | Units, directional measurements, fit errors, derivatives/objectivity and a stated calibration range |
| IO-01 | OBJ + MTL static asset round trip | `avatar.rs`, `garment/export.rs` | Units/axes, groups, UVs, normals and material assignments checked in independent tools; explicit losses |
| IO-02 | Static glTF/GLB import and export | `gltf_avatar.rs` (static avatar geometry import); canonical mesh/material model; separate direction-specific adapters | Document supported PBR/texture subset, transforms and units; independent reader/writer fixtures |
| IO-03 | DXF-AAMA and DXF-ASTM pattern exchange | `garment/dxf_import.rs` (bounded ASCII DXF geometry reader, no AAMA/ASTM semantics); stable pattern entities, grain and notch semantics | Separate dialect fixtures; correct physical dimensions; distinguish cut/sew lines and unsupported entities |
| IO-04 | Tiled pattern printing and cutting metadata | Physical SVG sewing outlines and 100 mm ruler available; printer output unverified | PDF/tiling, cutting allowances and labels; measured print tolerance |
| IO-05 | Animated cache and production exchange | Animation/time-sample model first | Alembic/USD, then FBX scope tested separately; stable topology policy, timestamps and external playback |
| QA-01 | Portable public benchmark pack | Small original garments and synthetic fixtures with provenance | Fresh checkout reproduces edit/save/reload and matched solver runs; large private captures are not required |

The owner reports useful collision robustness but residual wiggles and slow physical progress on finer meshes, including an 8 mm editing trial. This is user feedback, not a measured benchmark or a universal safety/performance claim. The next agreed dress benchmark has not yet been finalized. Keep historical captures labelled with their revision and settings.

## Next milestones, in order

16 October update: Open/Save/Save As, portable avatar assets and local recovery were implemented, followed by partial ranges and ordered 1:N/M:N sewing. See [document and sewing contracts](docs/documents-and-sewing.md). Multi-document tabs, asset cleanup, dedicated notch editing and manufacturing sewing semantics remain open. The rows below retain the broader acceptance goals.

17 November update: those document and sewing features shipped in the egui UI, which FC-0022 removed. The CPU APIs (`document.rs`, `fitting.rs`, `Command::{Sew, SewChain}`) remain, but the native editor only wires **Save Project**, which writes `<--output>/fitting-project.json` rather than the opened file (FC-0063 is in review), and whole-edge 2D sewing. P0.0 therefore comes first: P0.1 and P0.2 are not met in the shipped editor until it is done.

Each row is a contribution-sized track, not a promised release date. Finish one verifiable slice before adding another format or toolbar. The section 3 IDs (SAVE-, DRAW-, SEW-, ARR-, SIM-) are the canonical feature IDs; the milestones below name the IDs they group, and the UX- rows above track native-editor acceptance of the same features.

| Order | Outcome | Start in | Acceptance / evidence |
|---|---|---|---|
| P0.0 | Native editor parity for the core loop (UX-00) | `atelier/css_runner.rs`, [native UI status](docs/native-ui-status.md) | Every `[~]` item that P0.1 and P0.2 depend on is reachable in `cloth-fit --view`; the release-evidence T-shirt scenario passes there |
| P0.1 | Reliable save/open and portable projects (SAVE-01) | `fitting.rs`, `document.rs`, `atelier/css_runner.rs` | File picker, explicit save vs export, relative/relinked avatar paths, autosave recovery; reopen the same document/state after a forced shutdown |
| P0.2 | Understandable drawing and sewing (DRAW-01, DRAW-02, SEW-01, ARR-01) | `atelier/css_runner.rs`, `authoring.rs`, `garment/seam.rs` | Numeric dimensions, add/remove contour points, clear orientation and preflight intersections; a new user completes a top without editing JSON |
| P0.3 | Stable cloth at a measured realtime budget (SIM-01, PHYS-01, PHYS-02, PERF-01) | `gpu_avbd.rs` + `gpu_avbd/` (default); `ogc/`, `vbd.rs`, `gpu_vbd.rs` as controls | Shirt/apron/dress, supported and free cloth, multiple mesh densities; report step P50/P95, delivered physical time, seam gaps, stretch and intersections. Preserve accuracy checks |
| P1.1 | Practical static asset interchange | `avatar.rs`, `garment/export.rs`, future adapters | OBJ+MTL round trip, then static GLB; explicit units/axes, groups, normals and UVs; Blender and a second independent viewer agree |
| P1.2 | Industrial pattern exchange | canonical pattern model + new format adapters | DXF-AAMA and DXF-ASTM separately; import/export contours, grain and notches with independent fixtures; cutting vs sewing lines are explicit |
| P1.3 | Usable pattern printing | `garment/export.rs` | Physical SVG/PDF page dimensions, 100 mm ruler, tiling, labels and allowances; printed measurements agree within documented tolerance |
| P2.1 | Production animation exchange | future adapters + animation model | Fixed-topology Alembic cache; USD mesh/material/time samples; FBX avatar skinning/animation import; playback and unit/time tests in external DCCs |
| P2.2 | Truly replaceable solvers (PHYS-03, PHYS-04) | `fitting.rs`, `solver.rs`, `vbd.rs`, `gpu_avbd.rs`, `atelier/worker.rs` | One lifecycle/capability contract, two real adapters, shared scene suite; no fake backend selector. The native solver menu (WGSL AVBD/IPC, experimental WGSL AVBD/barrier AL, CPU VBD/OGC) predates this contract: the entries set a GPU flag and a `ContactMethod` that the worker branches on, not adapters behind a shared interface. Do not add entries until the contract exists |
| P3 | Manufacturing and richer appearance | pattern/gradation and material adapters | Named grading dialect, size rules and reversible grades; USDZ packaging; optional legacy caches and SBSAR baking |

The format priorities below remain product priorities: a **critical** animation format is still blocked by an animation/data-model milestone. Format names alone are not deliverables. Paired linked mirrors and saved 2D measurements are wired in the native editor ([scope](docs/panel-tools.md)). Friction, layered cloth, calibrated materials, same-panel symmetry and 3D surface measurements remain substantial tasks. Internal lines are wired natively; independent mirrors, holes (booleans and synchronized cutouts) and internal sewing exist only as CPU commands; see [their limits](docs/drafting-tools.md).

## 1. Draw → sew → arrange → simulate

- [x] Unified metric 2D board, rectangle/pen creation with straight and cubic Bézier boundaries; constrained Delaunay meshing.
- [x] Whole-edge 2D sewing (`Command::Sew`, `range: [0, 1]`) with seam removal and undo; easing is enabled only for unequal lengths and shown per seam.
- [~] Linked 2D/3D panel/edge selection, panel duplicate/delete, 3D edge sewing, direction handles and an explicit easing choice (FC-0061 in review).
- [~] Sparse colored seam links and endpoint ticks in 2D/3D, linked hover/selection and distinguishable pending previews (egui overlays removed).
- [~] World-axis translation/rotation gizmos, numeric transforms and transactional preview; the native Arrange accordion displays placement only.
- [~] Direct 3D move/rotate tools (G/R), edge-on rotation rings, 15° snapping and 2D board rotation independent of physical rest/grain.
- [x] Save editable templates/projects and reopen through the CLI; preserve v1/v2 data compatibility.
- [x] Drag existing anchors, toggle curved/straight points and adjust corner handles; board pan/zoom; corner fillets with live preview.
- [~] Snapping and saved 2D layouts (`Command::{BoardTransform, LayoutGroup}`).
- [x] Drag-drawn rectangles and paired linked mirrors.
- [~] Numeric primitives (rectangle, circle/ellipse, arc sector) with exact dimensions, independent X/Y mirrors, ±X/±Y copies and mirror unlink.
- [x] Internal lines.
- [~] Union/intersection/subtraction with holes and islands; internal Bézier sewing paths and eased ruffle strips (CPU validation only).
- [x] Multi-point selection and moves, point removal, unsewn boundary splits and line removal; Delete removes selected points.
- [~] Numeric point entry, rectangle resizing and target lengths (CPU validation; native interaction pending).
- [x] Light/dark theme, clean view and English UI.
- [~] Camera presets/orthographic views and board view rotation, adapted from FireflyTheReal; see [integration notes](docs/firefly-integration.md).
- [~] Radial tools, 2D panel/point marquee, group board movement and Ctrl+C/V with internal seams (the radial menu component exists but is not mounted).
- [ ] Same-panel symmetry, group rotation/3D transforms, 3D marquee, line offsets and dedicated hole point editing.
- [~] Import pattern geometry: DXF and SVG outline readers (`garment/dxf_import.rs`, `garment/svg_import.rs`, `pattern_import.rs`) have no native import control; FC-0064 (in review) handles the dead quick actions.
- [ ] Import verified reusable patterns (FreeSewing Teagan/Titan/Jett) with semantic sewing adapters ([investigation](docs/drafting-tools.md#web-pattern-library-investigation)).
- [~] Partial ranges in both views, numeric range editing, direction handles, ordered 1:N / M:N sewing (`Command::SewChain`) and seam editing.
- [ ] Dedicated notch editing, automatic correspondence and manufacturing sewing semantics.
- [ ] Arrangement points, body-aware placement and useful preflight intersection feedback.
- [x] Save Project with portable avatar assets: atomic write via `FittingProject::save`, avatar copied to the adjacent `.assets` folder. It writes `<--output>/fitting-project.json`, not the opened file (FC-0063 in review).
- [~] Native project picker, Open, Save As, dirty state, local recovery, avatar import and the template browser.
- [ ] Multi-document tabs, portable texture/material dependencies and managed recovery history.
- [ ] Seam allowance, grain annotations, graded sizes and print layouts.

Acceptance: an unfamiliar user can draft a simple top, join its intended boundaries, place it outside the avatar, simulate, save and reproduce the same editable garment. New formats must not store transient mesh indices as sewing references.

## 2. Physics as a replaceable module

- [x] Authoring and material-space model independent of the UI toolkit and wgpu; CPU-only build.
- [x] CPU Newton reference; CPU VBD; hybrid GPU VBD/OGC; real window timing and force checks.
- [x] Resident WGSL finite AVBD with IPC-style barrier contact (FC-0047), the desktop default; hardware checks against f64 material/sewing forces ([scope and limits](docs/wgsl-avbd-ipc.md)).
- [ ] Common backend lifecycle/capability contract, exercised by two real adapters. The native solver menu already exposes three entries without it; see P2.2.
- [ ] Standard scene suite and machine-readable quality/cost comparisons for alternative solvers.
- [x] First headless dress screening with VBD control, XPBD/OGC, finite-stiffness AVBD and a native Box3D lattice. [Results and failed gates](docs/solver-lab-results.md); no new editor backend promoted.
- [x] Conservative GPU BVH candidate search, explicit overflow retries and resident fixed geometry; exact contact geometry/bounds remain on CPU ([scope](docs/gpu-contact-search.md)).
- [x] Resident GPU contact for the AVBD/IPC backend: WGSL BVH refit and traversal, VF/EE barriers, contact decisions and all iteration passes on the device; one positions snapshot and a status block are read back per step.
- [ ] Rendering directly from solver buffers, qualified dense scenes and animated avatars; the hybrid VBD/OGC backend still computes exact contact geometry on the CPU.
- [ ] Friction, layered garments, better seam closure and calibrated material response.

The [18 October solver review](docs/solver-landscape-2026-10.md) proposes matched-model VBD/XPBD/compliant-ALM comparisons and separate external Newton/PPF evaluations, with a ruffled dress, small fixtures and matched pull/release controls. The saved Garment now has diagnostic and trajectory evidence; the [coupled seam experiment](docs/seam-block-refinement.md) failed its combined quality/performance gate and stays disabled. Next: complete-material-patch diagnosis and a reusable benchmark before another production correction. XPBD/OGC and finite-stiffness AVBD now have CPU research adapters; a separate native Box3D lattice is also tested. They are not selectable editor backends; the resident WGSL AVBD in `gpu_avbd/` is a separate implementation. A CPU Cosserat rod solver (`cosserat.rs`, FC-0051) handles explicitly authored rod geometry only; rod-based garments are the PHYS-04 research track. DAT and the other external candidates remain unimplemented. The [15 October review](docs/research-2026-10-15.md) remains historical background.

The challenge is **more accurate AND faster**, with disclosed tradeoffs. Keep authoring, import/export and benchmarks useful even if a future solver replaces today's engine. See [solver challenge](docs/solver-challenge.md).

## Interoperability

The priorities below are the product requirements. Within the critical group, implement small, validated slices instead of advertising a full format after parsing one example. Import and export are separate capabilities.

| Format | Industry use | Priority | Current support / next milestone |
|---|---|---|---|
| DXF-AAMA / DXF-ASTM | Industrial sewing pattern exchange | **Critical** | **Partial (geometry import, CPU only):** `garment/dxf_import.rs` reads model-space LINE/ARC/CIRCLE/LWPOLYLINE/POLYLINE with layers, INSUNITS and unsupported-entity issues; no AAMA/ASTM semantics, no export, no native import control. Next: notches, grain and piece IDs with independent AAMA/ASTM fixtures |
| OBJ + MTL | Static garments, avatars, Blender, Maya | **Critical** | **Partial:** avatar OBJ import (`cloth-fit --avatar`) and garment OBJ export (native **Export**); no MTL/material round trip. Add groups, material assignments and documented axes/UV conventions |
| glTF / GLB | Modern 3D assets, PBR, realtime visualization | **Critical** | **Partial (avatar import only):** `gltf_avatar.rs` loads static `.gltf`/GLB triangle geometry through `cloth-fit --avatar`; no garment export, materials, textures or animation. Next: static garment export, then a round trip |
| Alembic `.abc` | Animated simulation caches for VFX/cinematics | **Critical** | Planned. Fixed-topology position/normal cache, timestamps, units and DCC playback fixture |
| OpenUSD `.usd/.usda/.usdc` | Production pipelines and Chaos Cloth workflows | **Critical** | **Partial (static export, CPU only):** `usd_scene.rs` writes one static USDA garment scene ([scope and losses](docs/usd-scene.md); `examples/usd_static_scene.rs`); no reader, material networks or time samples. Next: time samples, then materials; validate any Chaos-specific schema separately |
| FBX `.fbx` | Rigged characters, animation, Unreal, Unity, DCC | **High** | Planned. Avatar hierarchy, skinning, bind transforms and sampled animation; select dependency/distribution strategy before integrating |
| PDF / SVG | Print and share 2D patterns | **High** | **Partial:** annotated preview and physical SVG sewing outlines with a 100 mm ruler; SVG outline import exists as a CPU reader (`garment/svg_import.rs`) without a native control. No PDF, tiling or allowances; physical printer validation remains open |
| Grading rule files | S/M/L/XL size ranges, industrial manufacturing | **High** | Planned. Identify concrete dialects; explicit units, size tables, stable point identities and reversible grade application |
| USDZ `.usdz` | AR and product presentation | **Medium** | Planned after USD: packaging, material/texture validation and viewer fixture |
| MDD / PC2 | Legacy deformation-cache pipelines | **Secondary** | Planned. Fixed vertex order, frame/time conversion and topology mismatch rejection |
| SBSAR | Procedural Substance materials | **Secondary** | Planned. Dependency/runtime evaluation and material baking workflow; appearance parameters are not calibrated cloth physics |

Every adapter must declare supported entities, units, axes/handedness, topology policy, material mapping and losses. Reject or report unsupported data instead of silently flattening it. Put adapters around the canonical garment model, not inside a solver. Add round-trip tests and open-source or self-authored fixtures with provenance. **Input and output have separate acceptance criteria** in [the interchange plan](docs/interoperability.md). Native CLO/Marvelous project files are not supported; do not infer editable garment support from mesh import.

## 3. Concrete authoring backlog from the manual study

The nine articles reviewed in the [dated manual ledger](docs/manual-study.md#authoring-backlog-review-2026-10-14) inform the tasks below. These are target acceptance criteria, not completed milestones; the Status column uses the legend at the top of this file. These IDs are canonical: the UX- and P- rows above refer to them rather than restating them. These are independent behavior specifications, not a plan to copy proprietary code or file formats.

| ID / milestone | Small deliverable | Dependencies / starting point | Done when | Status (17 Nov 2026) |
|---|---|---|---|---|
| SAVE-01 / P0.1 | Open, Save As, dirty-state feedback and portable avatar references | `fitting.rs`, `atelier.rs`; foundation for every later task | A saved shirt reopens from another directory with matching material/rest/state; missing avatar gives relink UI; cancelled or failed writes preserve the previous file | `[~]` CPU model done (atomic save, portable avatar); native editor has Save Project only (FC-0063 in review) |
| DRAW-01 / P0.2 | Rectangle width/height and numeric anchor movement in mm | `authoring.rs`, `atelier/css_runner.rs`; current metric model | A 400 × 600 mm rectangle keeps exact dimensions through save/load; invalid values are atomic; one edit is one undo entry | `[~]` Native rectangles are drag-drawn; numeric dimensions and anchor entry are CPU only (`Command::{Primitive, Dimensions}`) |
| DRAW-02 / P0.2 | Add/remove a contour point; split by arclength, ratio or count | DRAW-01; named-edge migration in `garment/curves.rs` and `authoring.rs`; MD-01 | Split a sewn Bézier without changing its outline; migrate seam ranges or reject explicitly; no dangling references after undo/remesh | `[~]` Point removal and unsewn splits wired; adding points and sewn-split seam migration open |
| SEW-01 / P0.2 | Interactive partial-edge sewing in both views with numeric length and endpoint handles | Existing `BoundaryRef.range` / compiler correspondence; `atelier/css_runner.rs`; MD-02 | Sew 100 mm of a 400 mm edge to another piece, reverse/edit/delete it, undo and reload; selected subranges and preview match the physical seam | `[~]` Ranges and range editing in the CPU model; native sewing is whole-edge 2D only |
| ARR-01 / P0.2 | Named body arrangement anchors with preview, normal direction and clearance diagnostics | `avatar.rs`, `authoring.rs`; MD-06 | Place front/back/sleeve panels on an arbitrary supported avatar; show offending pieces before simulation; retain 2D rest data and undo the placement | `[ ]` |
| SIM-01 / P0.3 | Edit persistent pins from 2D/3D and distinguish them from temporary mouse grabbing | Material-coordinate attachment model + solver capabilities; MD-07 | Pin/unpin a patch and resume; report what survives remeshing; no persisted triangle indices, hidden forces or silent conversion between hard and soft constraints | `[~]` CPU pins (`garment/pins.rs`); native Pins accordion is display only |
| DRAW-03 / P1 authoring | Mirror copies with optional linked outline/sewing edits, plus unlink | DRAW-02; explicit relation model; MD-03 | Edit one sleeve and update its counterpart exactly once; mirrored winding, grain and seam orientation remain valid; unlink and undo are deterministic | `[~]` Linked mirrors wired; independent mirrors and unlink (`Command::UnlinkMirror`) CPU only |
| DRAW-04 / P1 authoring | Internal lines, offsets and holes as distinct semantic objects | DRAW-02; topology/model migration and constrained mesher; MD-04 | A 10 mm internal offset remains attached to its piece; reject self-crossing offsets; a hole removes area/mass while a guide line does not | `[~]` Internal lines wired; holes CPU only; offsets open |
| SEW-02 / P1 authoring | 1:N, then M:N ordered sewing chains and editable notch correspondences | SEW-01; DRAW-04 for internal-line sewing; MD-05 | Sew two short edges to one long edge with visible ordering and explicit ease; cancel partial input; preserve totals, orientation and notches after remeshing | `[~]` 1:N and M:N chains in the CPU model (`Command::SewChain`); notches open |
| ARR-02 / P1 authoring | Fold placement preview about an internal line | DRAW-04 + ARR-01; MD-08 | Fold a cuff/hem and undo it without changing material rest; preflight initial intersections; distinguish placement from a physical crease constraint | `[ ]` |

Recommended follow-up: wire **SAVE-01** and **SEW-01** into the native editor (UX-00), finish their remaining acceptance work and add dedicated notch editing. Keep DRAW-02 and SEW-01 in separate reviewable changes. ARR-01 and physical performance work can proceed independently once they share the same document transactions. Do not block a usable save/draw/sew loop on automatic sewing, animation or visual trim tools.

### Release evidence, not feature counts

- A new user completes **T-shirt → added apron → simple dress** using only the UI, saves, closes and reloads each garment. Record the actual action sequence and any manual recovery.
- Every topology edit checks area/mass, stable semantic references, undo/redo and compatibility with old documents. Future schema changes need explicit migrations; do not silently change existing v1/v2 interpretation.
- For each physical backend, publish identical fixtures, settings and material data with seam gap, stretch, penetration diagnostics, step P50/P95 and simulated/wall time. Target >=0.95× physical time and >=55 UI frames/s on a named reference machine; this is an acceptance target, **not current universal performance**.
- Every import/export adapter must meet the separate direction-specific [interchange contract](docs/interoperability.md). Preserve the critical/high priorities above; manufacturing semantics and animation prerequisites determine implementation order.

## 4. Public-manual study queue

Next reading: darts/pleats, seam allowance, topstitch vs structural sewing, zippers/buttons, fabric testing/calibration, measurement, animation, retopology and UV/export details. Read each family before writing its implementation specification. Automatic sewing is later work, after explicit sewing and arrangement are reliable; generic AI automation is not a substitute for a correct garment model. Track observations, independent specifications, implementation status and acceptance cases in [manual-study.md](docs/manual-study.md). The full manual has not been exhaustively reviewed.
