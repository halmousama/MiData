# ARTCRAFT_INVESTIGATION.md — تحقيق: كيف بنى وكلاء AI تطبيقات بحجم PhotoCraft؟

> **تاريخ التحقيق:** 2026-10-27
> **المنهج:** قراءة مباشرة من المصادر الأولية (مستودع `storytold/photocraft`: ‏AGENTS.md كامل، docs/contributing.md، docs/control-protocol.md، docs/gaps.md، ROADMAP.md، واجهة GitHub API للبيانات الوصفية وسجل الـ commits، موقع getartcraft.com، نقاش Hacker News الأصلي) + مصدر ثانوي واحد موسّع (مقال greennode.ai التحليلي). كل اقتباس حرفي بالإنجليزية كما ورد في مصدره.
> **الغرض:** وثيقة مرجعية لمشروع إعادة بناء MiData بلغة Rust — استخلاص المنهجية التي مكّنت مهندساً واحداً + وكلاء AI من بناء 13 تطبيقاً معقداً، وإسقاطها على مشروعنا.
> **قاعدة التمييز:** ‏[م.أ] = مصدر أولي من المستودع نفسه · [م.ث] = مصدر ثانوي (مقال/نقاش).

---

## فهرس

1. [الملخص التنفيذي](#1-الملخص-التنفيذي)
2. [ما هو ArtCraft؟ الأرقام الكاملة](#2-ما-هو-artcraft-الأرقام-الكاملة)
3. [السؤال المركزي: ماذا فعل المهندس فعلاً؟](#3-السؤال-المركزي-ماذا-فعل-المهندس-فعلاً)
4. [الآلية 1: ‏AGENTS.md — دستور الوكلاء](#4-الآلية-1-agentsmd--دستور-الوكلاء)
5. [الآلية 2: معمارية طبقية مفروضة بأداة](#5-الآلية-2-معمارية-طبقية-مفروضة-بأداة)
6. [الآلية 3: كل شيء Command](#6-الآلية-3-كل-شيء-command)
7. [الآلية 4: نظام عمل مُقاس — الوكيل يختار مهمته بنفسه](#7-الآلية-4-نظام-عمل-مُقاس--الوكيل-يختار-مهمته-بنفسه)
8. [الآلية 5: الحلقة الخماسية وبوابات الإنهاء](#8-الآلية-5-الحلقة-الخماسية-وبوابات-الإنهاء)
9. [الآلية 6: التطبيق قابل للقيادة من الوكلاء](#9-الآلية-6-التطبيق-قابل-للقيادة-من-الوكلاء)
10. [الآلية 7: التوازي الآمن وقواعد الاشتباك](#10-الآلية-7-التوازي-الآمن-وقواعد-الاشتباك)
11. [طبقات الجودة الآلية والـ Ratchets](#11-طبقات-الجودة-الآلية-والـ-ratchets)
12. [النتيجة بصدق: ماذا تقول أرقامهم نفسها](#12-النتيجة-بصدق-ماذا-تقول-أرقامهم-نفسها)
13. [الخلاصات المستقطرة](#13-الخلاصات-المستقطرة)
14. [الإسقاط على MiData](#14-الإسقاط-على-midata)
15. [المصادر](#15-المصادر)

---

## 1. الملخص التنفيذي

**ArtCraft** (منظمة `storytold` على GitHub، شركة Storyteller.ai، بقيادة المهندس **Brandon Thomas**) هي عائلة من **13 تطبيقاً مفتوح المصدر مكتوبة بـ Rust خالص** تعيد بناء حزم Adobe وMicrosoft Office من الصفر (clean-room): محرر صور، رسم متجهي، مونتاج فيديو، إدارة مكتبة صور، PDF‏، مؤثرات حركية، نشر مكتبي، تصميم واجهات، معالجة نصوص، DAW‏، عروض تقديمية، CAD‏، وجداول بيانات — كلها بواجهة **egui/eframe على wgpu**، وكلها (تقريباً) تُصرَّف أيضاً إلى WebAssembly لنفس المتصفح.

الجواب المختصر على سؤال "كيف تمكن وكيل AI من بناء هذا؟":

> **المهندس لم يكتب الكود — بنى "مصنعاً" ثم وضع الوكلاء داخله.**
> المصنع = دستور قواعد (AGENTS.md) + معمارية طبقية يفرضها أمر `cargo xtask` + كل ميزة "أمر" قابل للاستدعاء آلياً + نظام وثائق عمل مُقاس (gaps/roadmap/scorecard/devlog) + حلقة مهمة خماسية تبدأ بالاختبار وتنتهي بلقطة شاشة + تطبيق قابل للقيادة برمجياً (قناة تحكم JSON + MCP) + توازٍ بملكيّات صارمة + بوابات آلية لا يمكن تجاوزها (clippy/CI/fuzz/corpus) + قائمة تصعيد مغلقة للبشر.
> الجودة لم تأت من ذكاء النموذج، بل من **استحالة تجاوز المصنع**.

الأرقام (من [م.أ] واجهة GitHub API بتاريخ التحقيق): المستودع أُنشئ **2026-09-30**، وبحلول 2026-10-26 بلغ **6,301 commits** و**311 فرعاً** و**~73,000 نجمة** و**12,900 forks** و**3,136 issues مفتوحة**، مع **1,004 PRs مدمجة** خلال الأسبوعين الأولين (معايرة التقديرات في ROADMAP.md [م.أ]). نحو **77%** من commits المستودعات السبعة الأولى تحمل Claude Opus 5.5 كـ co-author، وفي PhotoCraft وحده أضافت commits ‏Opus أكثر من **244,000 سطر** [م.ث: greennode].

---

## 2. ما هو ArtCraft؟ الأرقام الكاملة

### 2.1 التطبيقات الثلاثة عشر [م.أ: getartcraft.com/apps]

| # | التطبيق | المقابل التجاري | الحالة المعلنة |
|---|---|---|---|
| 01 | **PhotoCraft** | Photoshop | Early alpha |
| 02 | VectorCraft | Illustrator | In development |
| 03 | FilmCraft | Premiere Pro | In development |
| 04 | LightCraft | Lightroom | In development |
| 05 | PdfCraft | Acrobat Pro | Early alpha |
| 06 | EffectCraft | After Effects | In development |
| 07 | DesignCraft | InDesign | In development |
| 08 | FigCraft | Figma | Early alpha |
| 09 | WordCraft | Word | In development |
| 10 | SoundCraft | DAW (Audition/Logic) | In development |
| 11 | DeckCraft | PowerPoint | In development |
| 12 | CADCraft | AutoCAD | In development |
| 13 | GridCraft | Excel | In development |

بالإضافة إلى **ArtCraft Launcher** ‏(`storytold/craft-launcher`، توزّع كـ Flatpak) يثبّت التطبيقات كلها ويحدّثها. نموذج العمل: التطبيقات الـ13 مجانية مفتوحة المصدر، والشركة تربح من منتج منفصل هو استوديو ArtCraft للذكاء الاصطناعي (credits لاستدعاء النماذج) [م.ث].

### 2.2 PhotoCraft بالأرقام [م.أ: GitHub API + ROADMAP.md، تاريخ القراءة 2026-10-27]

| المقياس | القيمة |
|---|---|
| إنشاء المستودع | 2026-09-30 (أول ظهور عام 30 سبتمبر – 1 أكتوبر، الإطلاق أول أكتوبر) |
| Commits على main | 6,301 |
| PRs مدمجة | 1,004 (حتى معايرة 2026-10-17) — ترقيم issues/PRs وصل #9583 |
| الفروع | 311 (تسمية تحمل اسم الأداة: ‏`codex/egui-037-photo-followup-20261025`) |
| الوسوم | 18 |
| النجوم / المتابعون / Forks | 72,973 / 318 / 12,933 |
| Issues مفتوحة | 3,136 (منها 953 "قيد العمل" حسب ROADMAP بتاريخ 2026-10-13) |
| حجم الكود | ‏~92 MB للمستودع؛ +244,000 سطر من Opus 5.5 وحده [م.ث] |
| الترخيص | مساهمات MIT OR Apache-2.0؛ الشعارات غير مفتوحة |
| عرض الميزات vs Photoshop 2026 | **~76% مقاس**: قوائم 628/628، أدوات 56/68، لوحات 30/35، صيغ ملفات 13+4 من 31 |
| الجاهزية للعمل الحقيقي | **~45%** (تقدير موزون) — "mainstream practitioner ~43%"، "essentials user ~61%" |
| المتبقي لـ beta | ~1,100–1,800 **ساعة-وكيل** (Opus 5.5، وكيل واحد بتسلسل، معايرة من 1,004 PRs؛ ~80% من العمل قابل للتوازي) |
| المتبقي لتكافؤ كامل | ~2,300–4,100 ساعة-وكيل |
| أمانة PSD | ‏307 من 309 ملف اختبار psd-tools يُحفظ عرضها سليماً؛ "Photosson-oracle floor" متصاعد (145→190 من 258 في فترات مختلفة) |

**ملاحظة منهجية مهمة:** وحدات التقدير كلها "Opus 5.5 agent wall-clock hours" — أي أن الفريق يقيس العمل بوحدة "ساعات الوكيل" ويعايرها إحصائياً من تاريخ PRs المدمجة فعل. هذه أول مرة تُدار فيها خارطة طريق بوحدة قياس كهذه.

### 2.3 المكدس التقني [م.أ]

- **Rust فقط.** ‏"No JavaScript or TypeScript" — على الويب يولّد `wasm-bindgen` محمّلاً صغيراً؛ *"never hand-write JS"*.
- **الواجهة:** ‏egui 0.37 / eframe على wgpu ‏(Metal, Vulkan, DX12, WebGPU)‏. حظر صريح: *"Never add Tauri (or any webview/JS UI framework) as a dependency, build step or packaging target."*
- **Compositor مزدوج:** معالج CPU (هو الـ **oracle** المرجعي للاختبارات) ومعالج GPU ‏(wgpu) — النتائج تُقارَن بالمعالج المرجعي.
- **Web:** نفس الكود يُصرَّف إلى `wasm32-unknown-unknown` ‏(trunk + wasm-bindgen)، والقاعدة *"Never break wasm"* — فحص إلزامي `cargo xtask wasm` لكل الطبقات L0–L6.
- **بنية تحتية مشتركة بين التطبيقات الـ13:** مستودع `craftrules` (خاص: المعايير والدروس المشتركة، مثل `standards/never-crash.md` و`standards/fonts.md` و`standards/progress-docs.md` و`release/playbook.md`)، ومستودع `craft-libs` (مكونات Rust مشتركة **مثبّتة بـ git revision غير قابل للتغيير**)، ومستودع `craft-fonts` (الخطوط لا تُcommit أبداً في التطبيقات؛ تُمرَّر عبر `CRAFT_FONTS_DIR` وكل شيء يجب أن يعمل بدونها)، ومستودع `photocraft-corpus` (ملفات الاختبار الحقيقية المرجعية).
- **أدوات الجودة:** ‏CodeRabbit (مراجعة كل PR)‏، SonarCloud (وتظهر أرقام قواعده في رسائل commits مثل S8855/S1612)، Codebuff (وكيل إضافي ضمن المزيج)، commits البشر موقّعة GPG.
- **مزيج النماذج:** ‏Claude Opus 5.5 (حصان العمل ~77%)‏، Claude Sonnet 5.5‏، Claude Fable 5.1‏، OpenAI Codex‏، GitHub Copilot‏ — *"Choosing the right model for the right job… is becoming a core skill"* [م.ث].

### 2.4 من خلفه؟

- **Brandon Thomas** — مهندس برمجيات، يقود Storyteller.ai؛ صرّح على HN: *"Everything can be open source multiplatform Rust now."* [م.ث] وحسب تعليق في نقاش HN فإنه *"لم يكتب سطراً من الكود منذ فبراير 2026 — كله مولّد بالـ AI"* [م.ث: نقاش HN، تعليق dofm].
- مساهمون بشريون ظاهرون في سجل الـ commits (تواقيع GPG): ‏Dylan M. Taylor ‏(dylanmtaylor)، skylar taylor-barrick، وآخرون — أدوارهم: الدمج، المراجعة، الترحيلات الكبيرة (مثل ترحيل egui 0.37)، والقرارات.
- الفريق يرفض صراحةً الهندسة العكسية/فك الترجمة للبرامج التجارية (رفض استخدام أداة REA) التزاماً بمبدأ clean-room ‏[م.ث].

---

## 3. السؤال المركزي: ماذا فعل المهندس فعلاً؟

تحليل المصادر الأولية يظهر أن عمل البشر انقسم إلى **خمسة أدوار لا تتضمن كتابة الميزات**:

1. **كتابة الدستور** — ‏AGENTS.md وقواعد craftrules: ما الممنوع، ما المطلوب، كيف تُبنى الميزة، كيف تُختبر، متى يُصعَّد القرار.
2. **تعريف العمل وترتيبه وقياسه** — ‏gaps.md (قائمة مرتبة)، ROADMAP.md (رقمان رئيسيان + أبعاد)، scorecard (أرقام مولّدة آلياً)، parity checklists لكل بعد.
3. **بناء آلات الإنفاذ** — `cargo xtask` بأوامره (layers/wasm/parity/scorecard/perf/corpus/ci)، clippy.toml، ‏panic_hunt، الـ corpus oracles، الفحص البصري offscreen، قناة التحكم. القاعدة: **كل قاعدة مهمة يقابلها أمر آلي يرفض مخالفتها** — لا شيء يعتمد على "انتباه الوكيل".
4. **الفصل في القرارات الخطرة** — قائمة تصعيد مغلقة (تراخيص، مال، نشر، بيانات شخصية، git مدمر، ما لم تغطّه القواعد). وما عداها: *"Don't block on choices you can make yourself."*
5. **التخطيط والإصدار والتوقيع** — تبقى بشرية: التخطيط الكلي، قطع الإصدارات، أسرار التوقيع، تغليف المثبّتات.

> الخلاصة كما صاغها المقال التحليلي [م.ث]: *"Humans are no longer the ones writing code. They write the rules, divide the work, set the standards, and approve risky decisions. AI agents handle the rest."*

---

## 4. الآلية 1: ‏AGENTS.md — دستور الوكلاء

ملف واحد (19KB)‏ في جذر المستودع، أول جملة فيه توصي: *"Read this file first, then docs/"*. بنيته [م.أ]:

### 4.1 جدول التوجيه ("Orientation — 5 minutes")

جدول يربط كل وثيقة بسبب قراءتها: ‏`docs/architecture.md` (خريطة الـ crates والطبقات و"درز المحرك/الواجهة")، `docs/development.md` (البناء والاختبار و**قيادة التطبيق برمجياً**)، `docs/contributing.md` (القواعد + checklist "أضف أمراً")، `docs/control-protocol.md` (قناة JSON)‏، `docs/ui-design.md` (رموز التصميم والثيمات)، `ROADMAP.md`، `docs/target-app-parity.md` (التقييم المرجعي الموزون بالأدلة)، `docs/gaps.md` (**"the work list to pick from"**)، `docs/roadmap.md` (التركيز الحالي)، ملفات parity لكل بعد، `docs/parity-checklist.md` (مولّد)، `docs/scorecard.md` (مولّد)، README لكل crate‏، ومستودع الـ corpus.

### 4.2 القاعدة الذهبية: ‏"Never crash (outranks feature work)"

حرفياً: *"People trust PhotoCraft with their work, and a crash loses it. A malformed file, a bad command or MCP param, a corrupt settings file, an odd keystroke or a full disk must produce an error the user or agent can act on, never a panic. Don't ship a feature by adding a panic path; fix a crash before building on top of it."*

تفصيلاتها القابلة للإنفاذ:

- **Non-test code never panics:** ممنوع `unwrap()`‏/`expect()`‏/`panic!`‏/`unreachable!`‏/`todo!`‏/`unimplemented!`. الأخطاء عبر نوع error الـ crate مع `?`‏، وبدائل `ok_or(..)?` و`let..else` و`if let` و`unwrap_or*` فقط حيث البديل صحيح حقاً (*"never one that silently corrupts a document"*). الميزة غير المكتملة ترجع خطأ "unsupported". الاستثناء الوحيد: حرفية قابلة للإثبات بعدم الفشل مع `#[allow(clippy::expect_used)]` و`.expect("why it can't fail")`.
- **No unsafe:** ‏`unsafe_code = "forbid"` على الـ workspace. الاستثناء الوحيد crate معزول لإدخال أقلام الرسم (`tablet`): ‏`deny` داخلياً، كل كتلة مع تعليق `SAFETY:`، واختبار ضد أحداث `NSEvent` ورسائل pointer حقيقية.
- **Input-derived numbers are hostile:** ‏`get()` بدل `[i]`/`[a..b]`، قصّ النصوص على حدود char فقط، `checked_*`/`saturating_*` للأطوال والإزاحات، حماية قسمة على صفر و NaN/inf‏، سقف للتخصيصات المحسوبة من مدخلات.
- **Bound recursion** بحدود عمق أو seen-sets (المستندات قد تكون عميقة أو دائرية).
- **Don't cascade:** معالجة تسمم الأقفال `lock().unwrap_or_else(PoisonError::into_inner)` واعتبار thread joins نتائج `Result`.
- **Last-resort guard:** صدفة التطبيق تلتقط أي panic شارد حول تنفيذ الأوامر واستيراد/تصدير الملفات، تحوّله خطأً، وتحفظ المستند. *"It's a safety net, not a licence to panic."*
- **Prove it:** كل إصلاح crash يأتي مع اختبار انحدار تركيبي صغير كان ينهار قبل الإصلاح.
- **Enforced by clippy:** ‏`clippy.toml` يسمح بـ unwrap/expect/panic/indexing **في الاختبارات فقط**، والـ crates النظيفة تحمل `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.

### 4.3 بقية القواعد الذهبية (منتقاة) [م.أ]

- **No format or colour assumptions:** عمق البت (8/16/32f) ونموذج اللون (RGB/Gray/CMYK/Lab…) **بيانات runtime**‏؛ ممنوع مسار pixels بـ u8 فقط في APIs العامة؛ ممنوع افتراض sRGB — التحويلات عبر `photocraft-cms`‏؛ الاختبار على عدة أعماق.
- **Clean-room:** الدراسة للسلوك والمظهر فقط، من المواصفات العامة (Adobe PSD spec‏، ICC‏، ISO 32000) والملاحظة؛ لا نسخ كود/shaders/ملفات تعريف/أصول؛ كل أصل بترخيص متسامح + ملف ترخيصه بجانبه + سطر في `ATTRIBUTION.md` **ضمن نفس التغيير**؛ خصوصية حتى في جلب الأصول: ممنوع وضع أسماء/إيميلات بشر في الطلبات، وUser-Agent عام `Photocraft-dev`.
- **Tests are the gate:** كل تغيير معه اختبارات؛ crates الصيغ: round-trip + مولّدات تركيبية + oracles + fuzz‏؛ ممنوع انحدار نتائج corpus الـ PSD أو "parity floor".
- **The UI is thin and data-driven:** حالة الواجهة في `ui-egui/src/state.rs` (serde) **حتى تستطيع قناة التحكم قراءتها وقيادتها**؛ الألوان والانحناءات من `theme::Tokens` ولا تُثبَّت يدوياً أبداً.
- **Verify UI changes visually:** ارسم offscreen بـ `cargo run -p photocraft-ui-egui --example snapshot` ‏(*"no window, no focus stealing"*) أو أقلع بـ `--control` والتقط `ui.screenshot`. ‏**"Look at the PNG."** صور العرض فن ملك عام فقط.
- **Performance is a feature:** قياس العمليات الثقيلة على صورة 24–36MP ببناء release‏؛ العمل لكل tile بتوازٍ (rayon)؛ تجاوز الـ tiles الفارغة؛ ممنوع مسح سطح كامل لكل إطار (cache لكل revision)؛ **تسجيل توقيتات قبل/بعد في devlog**.
- **Never panic on input** + اختبار `panic_hunt` يفزّز كل الأوامر بمدخلات معادية ويجب أن يبقى أخضر.
- **Time goes through chrono:** ممنوع لفّ تحويلات التقويم يدوياً (خوارزميات days-to-date وحساب السنين الكبيسة)؛ الاستثناء الوحيد موثّق بدقة (تمديد المدى بدورات غريغورية 400 سنة = 146,097 يوماً).

---

## 5. الآلية 2: معمارية طبقية مفروضة بأداة

خريطة الـ workspace [م.أ: AGENTS.md §2]:

```text
crates/
  geom cms color raster      L0 أساسات (هندسة، إدارة ألوان ICC، صيغ pixels + رياضيات المزج، COW tiles)
  psd codecs                 L0 صيغ ملفات مستقلة (بلا تبعيات workspace — قابلة للنشر)
  tablet                     L0 إدخال أقلام الرسم (الـ crate المعزول الوحيد المسموح فيه unsafe)
  doc                        L1 نموذج المستند (طبقات، أقنعة، تعديلات، تأثيرات، smart objects — بيانات نقية)
  ops paint algo text vector L2 السجل/التاريخ، محرك الفرش، خوارزميات المعالجة، محرك النصوص، المسارات/الأشكال
  compose gpu format         L3 معالج CPU (الـ oracle)، معالج wgpu، صيغة .pcraft الأصلية
  io plugins                 L4 مستند <-> PSD/الصيغ المسطحة؛ إضافات WebAssembly معزولة (sandboxed)
  engine                     L5 Session + سجل الأوامر (كل فعل هو command)
  ui-egui automation         L6 صدفة egui (رقيقة: كل الأفعال عبر المحرك)؛ خادم MCP
  testkit                    مساعدات اختبار
apps/
  photocraft                 تطبيق سطح المكتب (eframe/wgpu) + خادم تحكم TCP
  photocraft-cli             CLI بلا رأس (convert/info/run/batch/commands/mcp)
  photocraft-web             نفس التطبيق في المتصفح (trunk + wasm-bindgen)
  photocraft-web-boot        إقلاع نسخة الويب من أجزاء مضغوطة
xtask/                       cargo xtask layers | wasm | ci | stats | corpus | test-corpus | parity | perf | scorecard
```

**الإنفاذ:** *"Layering is enforced by `cargo xtask layers`. A crate may depend only on lower layers. Nothing below `ui-egui` may use egui, eframe, winit or rfd. A new crate must be registered in `xtask/src/layers.rs`."* — أي أن "ممنوع تسرب الواجهة إلى المنطق" ليس عرفاً بل فحصاً يفشل البناء.

**لماذا هذا حاسم للوكلاء؟** لأنه يحوّل قرارات المعمارية من "سياق يجب أن يتذكره الوكيل" إلى "قيد يفشل الأمر بسببه" — الوكيل لا يحتاج أن يفهم لماذا، فقط أن يطيع البوابة.

---

## 6. الآلية 3: كل شيء Command

النص الحاكم [م.أ: AGENTS.md §3.1]: *"Everything is a command. New user-visible behaviour = a command in the engine (`crates/engine/src/*_cmds.rs`, registered in `commands.rs`) with id, label, menu path, shortcut, params doc, `enabled` and `run`, plus tests. The UI, CLI, control channel and MCP all dispatch commands by id."*

تشريح الأمر الواحد:

| الحقل | الوظيفة |
|---|---|
| `id` | معرف فريد بنمط `layer.smartFilter.delete` — **نفس الـ id من `menu_catalog.rs` يجعل خانة قائمة Photoshop تضيء تلقائياً بلا أي عمل UI** |
| `label` / `menu` / `shortcut` | التسمية ومسار القائمة والاختصار الافتراضي |
| params doc | توثيق المعاملات بصيغة يقرؤها الوكلاء: ‏`{"radius":px=4,"mode":"a|b"="a"}` — تُعرض عبر `commands`/`command_list` |
| `enabled` | مسند (predicate) يرمّد خانة القائمة عندما لا ينطبق الأمر |
| `run` | ينفّذ ويرجع **JSON result** ويسجل **خطوة تاريخ (undo) واحدة بالضبط** — مع `coalesce` للسحب وجلسات الكتابة |

قواعد مرافقة: الخوارزمية نفسها تسكن **أدنى crate مناسب** (algo/paint/vector/text/cms) مع unit tests‏، وتعمل على أسطح محايدة للعمق، لكل tile‏، وحتمية (randomness ببذرة). الأمر يحترم السياق (التحديد الناعم، الطبقة vs القناع، الأقفال، نموذج اللون، العمق). حالة العرض/النوافذ فقط (zoom، panels، screen mode) تبقى في الصدفة.

**checklist "إضافة أمر"** من contributing.md (7 خطوات): اعثر على الـ id ← الخوارزمية في أدنى crate مع unit tests ← الأمر في وحدة محرك جديدة (`<area>_cmds.rs` مع `specs()`) ← احترم السياق ← اختبارات (سلوك، undo/redo، حالات التعطيل، معاملات سيئة، أعماق متعددة) ← حوار schema-driven بمعاينة حية إن كانت هناك معاملات ← شغّل `cargo xtask parity` وcommit الملف المولّد.

**الأثر المركّب:** لأن كل سلوك = أمر موثّق قابل للاستدعاء بـ id:‏
1. الـ UI رقيق (أزرار تستدعي ids) — الوكيل لا يحتاج فهم egui ليضيف ميزة.
2. الـ CLI يأتي مجاناً (`photocraft-cli run <command>`).
3. قناة التحكم وخادم MCP يلفّان نفس السجل — **الوكيل يقود التطبيق كما يقوده الإنسان تماماً**.
4. الاختبار الشامل ممكن: ‏`panic_hunt` يمر على **كل** الأوامر بمدخلات معادية.
5. التكافؤ مع Photoshop يصبح **قابلاً للعد**: ‏628/628 بند قائمة مربوطة بأوامر.

---

## 7. الآلية 4: نظام عمل مُقاس — الوكيل يختار مهمته بنفسه

هذا هو القلب الإداري. الوثائق ليست وصفية بل **تشغيلية** — كل واحدة تجيب سؤالاً لدى الوكيل:

### 7.1 ‏`docs/gaps.md` — قائمة العمل المرتبة [م.أ]

- 17 فجوة (G1–G17)‏، *"one entry each, ranked by user impact on the way to beta"*.
- لكل فجوة: النوع (file format/performance/stability/UI/features/ecosystem/hardware/localization/AI/platforms)، الأثر (**blocks beta** / high / medium / low-medium)، تقدير ساعات-وكيل، رابط وثيقة التكافؤ التفصيلية، وعلامة **"Human"** لما لا يستطيع الوكيل إنهاءه وحده.
- مثال (G1: أمانة PSD وقبول Photoshop — blocks beta — 150–250h؛ G2: الأداء التفاعلي — blocks beta — 200–350h؛ G3: backlog أخطاء المستخدمين — blocks beta — 250–400h … حتى G17: المنصات والتوزيع — 30–60h + human).
- التعليمات الحرفية للوكيل: *"This is the work list agents pick from: take the highest-ranked entry that nobody is working on, and when you close part of one, update its evidence and estimate here and the matching parity doc."*
- ترويسة الملف **سجل تغييرات حي** بتواريخ وأرقام PRs/issues لكل تحديث ("G11: a recorded Save As imports with its format and file…, #7102؛ G9: … 24 of 169 preferences do nothing؛ G2: a cancelled 24 MP Gaussian Blur job stops in at most 44 ms…").
- أرقام مرجعية تتصاعد ولا تنزل: "psd-tools oracle floor 242/309"، "Photoshop-oracle floor 190/258".

### 7.2 ‏`ROADMAP.md` — صفحة الأرقام الرئيسية [م.أ]

رقمان رئيسيان (عرض الميزات ~76% **مقاس جزئياً** / الجاهزية ~45% **تقدير موزون**) + جدول أبعاد (Features 55%، UI/UX 40%، File formats 60%، Hardware 50%، Localization 55%، Performance 25% — 3/25 ميزانية محققة، Stability 45%، Platforms 80%، Ecosystem 15%، AI 10%) + جدول مناطق الميزات + **فقرة "Why alpha"** تشرح صراحة سبب عدم الجاهزية (953 issue، 22/25 ميزانية أداء فاشلة، PSD غير موثوق) + شروط beta ‏(~75% + لا فجوة PSD حاجبة) + "Upcoming" مرتب + ختم حوكمة: ‏**"Last reviewed / Last updated / Change / Target"** مع سجل مراجعة.

### 7.3 ‏`docs/scorecard.md` و`docs/parity-checklist.md` — **مولّدة لا مكتوبة**

- الـ scorecard يولّده `cargo xtask scorecard` من: ‏`scorecard/*.toml`‏، `perf/budgets.toml`‏، `perf/baseline.json`‏، حدود الـ corpus‏، وتدقيق "الإعدادات التي لا تفعل شيئاً". القاعدة: *"Its numbers are measured; prefer them to estimates."*
- الـ parity-checklist يولّده `cargo xtask parity` — كل بند قائمة في Photoshop حيّ أو مفقود، والوكيل **يُلزم بـ commit الملف المولّد**، وCI يتحقق من تطابقه.
- `docs/target-app-parity.md` يحمل المنهجية والأوزان والأدلة وساعات المعايرة.

### 7.4 ‏`log/devlog.md` — ذاكرة الاستمرارية بين الجلسات [م.أ]

*"Then append a terse entry to `log/devlog.md` (what landed, numbers, what's still open). **Sessions can end abruptly (crashes, context limits), so the dev log plus a green tree is how the next agent picks up.** Keep the tree building at every step."* — هذا هو الحل الصريح لمشكلة انقطاع سياق الوكيل: الحالة الحقيقية تعيش في (1) سجل نصي مختصر + (2) شجرة بناء خضراء، لا في ذاكرة الجلسة.

### 7.5 ‏`plan/` — التخطيط المحلي (gitignored)

أبحاث، خطة تكافؤ، خطة تنفيذ، تقديرات — منفصل عن `log/` وكلاهما محلي لا يُcommit (يبقى المستودع نظيفاً من عمليات التفكير، وتبقى الوثائق العامة هي العقد).

### 7.6 ترتيب الأولويات وقاعدة الـ Ratchet

*"Priorities: important infrastructure first, then low-hanging parity, then the long tail."* — وتسلسل اختيار العمل مرقّم: ‏ROADMAP ثم gaps ثم scorecard ثم "Current focus" ثم `xtask parity` (الفريسة السهلة عادة = أمر مفقود خوارزميته موجودة أصلاً في algo/paint/vector/text) ثم "Still open" في devlog.
**Ratchet:** *"When parity rises, raise `FLOOR` in `crates/ui-egui/src/parity.rs` (never lower it)."* — الجودة تتراكم أحادياً ولا تُسترجع.

---

## 8. الآلية 5: الحلقة الخماسية وبوابات الإنهاء

### 8.1 الحلقة (لكل مهمة) [م.ث — greennode، متسقة مع [م.أ]]

1. **Orient:** اختر المهمة التالية المفتوحة واقرأ وثائق المعمارية للكود المعني.
2. **Tests first:** *"Write down the acceptance test before writing code"* — تعريف "منتهي" قبل أي كود.
3. **Implement & run tests.**
4. **Verify:** شغّل CI‏؛ ولميزات الواجهة: **أقلع التطبيق بوضع التحكم، انقر واسحب عبر الميزة بنفسك، والتقط لقطات شاشة وافحص النتيجة** (عبر offscreen snapshot أو `ui.screenshot`).
5. **Record:** حدّث الوثائق والخارطة وdevlog، وcommit برقم المهمة، وانتقل للتالية.

### 8.2 بوابة "قبل أن تنهي مهمة" [م.أ: AGENTS.md §5 — حرفياً]

```sh
cargo test -p <crates you touched>
cargo clippy -p <crates> --all-targets -- -D warnings
cargo xtask layers
cargo xtask wasm            # إذا لمست L0–L6
cargo xtask parity          # إذا أضفت أوامر؛ وcommit الملف المولّد
cargo test -p photocraft-engine --test panic_hunt -- --ignored   # إذا أضفت/عدّلت أوامر
cargo xtask scorecard       # إذا حرّكت رقماً؛ وcommit الملف المولّد (CI يتحقق)
cargo xtask perf --quick    # إذا لمست مساراً ساخناً
cargo xtask test-corpus     # إذا لمست psd/io/codecs/compose/gpu/text/format
```

مع إرشادات تسريع الحلقة أثناء التكرار (`--lib --tests` لتخطي الأمثلة وdoctests، `--test <file>` لملف واحد، `cargo nextest run` للتوازي) — والبوابة الكاملة تبقى إلزامية في النهاية.

### 8.3 الـ Test Corpora والـ Oracles [م.أ + م.ث]

- ملفات حقيقية في `corpus/` ‏(gitignored، لا تُcommit أبداً)، تُجلب بـ commits مثبتة (pins في `xtask/src/corpus_pins.rs`) وتُحقَّق sha256 عبر `cargo xtask corpus --all`: ‏**oracles من صنع الفريق** (ملفات PSD حُرّرت في Photoshop فعلاً وXCF من GIMP) في مستودع `photocraft-corpus` + مجموعات upstream ‏(psd-tools, ag-psd, PngSuite).
- اختبارات الـ corpus اختيارية الميزة (`corpus` feature): ‏`cargo test` العادي يتخطاها، وCI **يشغّلها دائماً** (job مخزّن بالـ pin).
- FilmCraft يستخدم **FFmpeg كمرجع اختبار** (مقارنة المخرجات) لا كمكوّن داخل التطبيق [م.ث].
- المعالجات المرجعية الداخلية: compositor الـ CPU هو oracle لمعالج GPU‏؛ الاختبارات التركيبية round-trip للصيغ.

### 8.4 تعريف "منتهي" (Definition of Done) [م.ث]

الميزة تُعد منجزة فقط عندما: مسجّلة كأمر موثّق + مغطاة باختبارات (بما فيها undo/redo) + مربوطة بالقوائم والاختصارات + **قادها الوكيل وراجع لقطات الشاشة** + CI أخضر + الوثائق محدّثة.
*"This standard is what keeps thousands of AI-written commits at a minimum level of quality without anyone reading every line of code."*

---

## 9. الآلية 6: التطبيق قابل للقيادة من الوكلاء

شعار المستودع: *"Built for agents — Every menu item, anything you can click, a script or an AI agent can do too."*

### 9.1 بروتوكول التحكم [م.أ: docs/control-protocol.md]

- خادم TCP على **‏127.0.0.1 فقط** (loopback)‏؛ الإقلاع بـ `--control 7878 --control-token-file <path>`‏؛ رمز 256-bit في ملف بصلاحيات 0600 (يُنشأ تلقائياً إن غاب)؛ أوامر الاستخدام الخاطئة تخرج بكود 2.
- أول طلب في كل اتصال **مصادقة إلزامية**: ‏`{"method":"auth","params":{"token":"<64 hex>"}}` — لا يُنفَّذ أي شيء قبلها.
- بعدها كل طلب سطر JSON واحد بجواب سطر JSON واحد بنفس `id`‏؛ مهلة 60 ثانية **بدلالات دقيقة**: ‏`timeout: not run` (ما زال بالطابور — آمن لإعادة الإرسال) مقابل `timeout: no reply in 60s` (نُفّذ وقد يسري مفعوله — تحقق من المستند قبل الإعادة). حتى سلوك Wayland موثّق (النافذة المحجوبة تجعل الطلبات تنتهي بمهلة والتطبيق سليم).
- **الدوال:** ‏`engine.execute {command, params}` (تنفيذ مباشر بلا حوارات)‏، `engine.commands` (القائمة مع حالات التفعيل)‏، `ui.inspect` (**حالة الواجهة كاملة**: الأداة، اللوحات، المشاهد، الحوارات، النوافذ، الأحجام، تفضيلات العرض، معلومات GPU‏ — ممكنة لأن حالة UI‏ serde في `state.rs`)‏، `ui.menu.invoke` (سلوك النقر الحقيقي بما فيه فتح الحوارات)‏، `ui.menu.list`‏، `ui.screenshot`‏، `ui.set`‏، `tools.setColors`…
- **خادم MCP** ‏(`photocraft-cli mcp --bridge 127.0.0.1:<port>` من crate ‏`photocraft-automation`) يلفّ **نفس البروتوكول** — فوكلاء المستخدم النهائي (Claude Desktop‏، Cursor…) يقودون التطبيق بنفس الطريق الذي يتحقق به وكلاء التطوير.

### 9.2 التحقق البصري

- ‏`cargo run -p photocraft-ui-egui --example snapshot` — رسم offscreen ‏(*"no window, no focus stealing"*) بلا حاجة لشاشة.
- أو التطبيق الحي بوضع التحكم + ‏`ui.screenshot`.
- القاعدة الآمرة: **"Look at the PNG."** — الوكيل ينظر فعلاً إلى الصورة ويتحقق، ويرفق before/after screenshots في الـ PR ‏[م.أ: contributing.md].

### 9.3 لماذا التصميم agent-native؟ (الغرض المزدوج) [م.ث]

1. **أثناء التطوير:** هو وسيلة الوكيل لفتح التطبيق وتشغيل الميزة والتحقق منها ذاتياً (إغلاق الحلقة بدون بشر).
2. **للمستخدم النهائي:** أتمتة حقيقية — مثل تصحيح ألوان دفعة صور وفق قاعدة عبر MCP.

> *"In other words, the software is built so that AI can write it, test it, and use it."*

---

## 10. الآلية 7: التوازي الآمن وقواعد الاشتباك

### 10.1 العزل والملكية [م.أ: AGENTS.md §6 + م.ث]

- كل وكيل في **git worktree خاص** به، و**كل crate له مالك واحد بالضبط** — فلا يكتب وكيلان في نفس المكان [م.ث].
- مجلد بناء خاص: ‏`CARGO_TARGET_DIR=target/agent-<name>` لتجنب قفل Cargo ‏(وكل target dir ‏~10GB — حذف `target/agent-*` للوكلاء المنتهين).
- **الملفات المشتركة** (`engine/src/lib.rs`، قائمة `v.extend(...)` في `commands.rs`، `menus.rs`، `state.rs`): تعديلات *"small, surgical"* مع **إعادة قراءة قبل التحرير**.
- الأوامر الجديدة في **وحدات جديدة** (`engine/src/<area>_cmds.rs` مع `specs()`) *"rather than growing a shared file"* — تقليل النزاعات بنيوياً.
- كل `Cargo.toml` صالح دائماً (الـ glob `crates/*` يعني manifest مكسور = كسر بناء الجميع): **الكتابة ذرّية** — ملف مؤقت خارج `crates/` ثم `mv`.
- إذا كسر تعديلُ وكيلٍ آخرَ البناءَ: **انتظر وأعد المحاولة؛ لا تصلح ملفاته**.
- مسارات دمج دورية تظهر في السجل: ‏`Integration: track t04` مع قوائم conflicts محلولة — أي أن هناك "قضباناً" (tracks) متوازية تُدمج بانتظام، و311 فرعاً بأسماء تحمل بادئة الأداة (`codex/…`).

### 10.2 استقلالية الوكيل وتصعيد البشر [م.ث]

- **"Don't block on choices you can make yourself"** — الوكيل يحسم كل ما تغطيه القواعد.
- يتوقف ليسأل بشراً **فقط** في: تراخيص خارج القائمة المسموحة · إنفاق مال · النشر أو الدفع للمستودع الرئيسي · البيانات الشخصية · عمليات git مدمرة (force push) · أي شيء لا تغطيه القواعد بوضوح — بمبدأ **"when in doubt, leave it out"**.
- يبقى بشرياً: التخطيط الكلي، الإصدارات، توقيع المثبّتات.

### 10.3 الاعتماد المتبادل بين مستودعات متعددة

معايير ودروس مشتركة في `craftrules` (خاص)‏، ومكونات Rust مشتركة في `craft-libs` **مثبّتة بـ git revision غير قابل للتغيير** — الوكيل في تطبيق لا يستطيع كسر تطبيق آخر عبر تحديث عائم.

---

## 11. طبقات الجودة الآلية والـ Ratchets

| الطبقة | الآلية | الأثر |
|---|---|---|
| المترجم | ‏`unsafe_code = "forbid"`‏؛ حيلة `photocraft-format`: **يفشل التصريف عمداً** إذا اكتسب struct المستند حقلاً جديداً دون تحديث الصيغة — *"nothing is silently dropped from .pcraft saves"*‏؛ الحقول الجديدة بـ `#[serde(default)]` فتُقرأ الملفات القديمة؛ كتل PSD المجهولة تُحفظ verbatim | استحالة نسيان مفروضة بالمترجم |
| الـ linter | ‏clippy.toml + ‏`#![deny(unwrap_used, expect_used, panic, …)]` + ‏`-D warnings` في البوابة؛ قواعد Sonar تُعالَج في commits مخصصة (S8855…) | قاعدة Never-crash آلية |
| الاختبارات | لكل تغيير؛ round-trip+fuzz للصيغ؛ 8/16/32-bit للبكسل؛ graceful-failure test لكل أمر جديد؛ undo/redo‏؛ `panic_hunt` لكل الأوامر | "منتهي" = مُختبَر |
| الـ oracles | ‏corpus ملفات حقيقية مثبّتة sha256 ‏+ compositor CPU مرجعاً لـ GPU‏ + FFmpeg لـ FilmCraft‏ + حدود أرضية تتصاعد (307/309، 190/258…) | صحة مقابل الحقيقة لا مقابل الذات |
| الأداء | ‏`perf/budgets.toml` + ‏`baseline.json` + ‏`xtask perf`‏؛ الميزانية تصبح `enforce = true` عند تحقيقها | انحدار الأداء يُرصد رقمياً |
| المراجعة | CodeRabbit على كل PR‏؛ SonarCloud؛ humans يدمجون (GPG)‏؛ ترحيلات كبيرة بقيادة بشرية (egui 0.37) | عين ثانية آلية + قرار بشري |
| CI | بوابات صارمة (*"ci: integrate current main while retaining strict test gates"*)‏؛ الملفات المولّدة (parity/scorecard) يتحقق CI من تطابقها | لا دمج أحمر |
| Ratchets | parity FLOOR في الكود *"never lower it"*‏؛ corpus floors‏؛ perf enforce‏؛ وثائق بحقول "Last reviewed/Change" وسجل مراجعة | الجودة أحادية الاتجاه |

---

## 12. النتيجة بصدق: ماذا تقول أرقامهم نفسها

- الحالة: **alpha** — الفريق نفسه يعلن: 953 issue مفتوحة، 22 من 25 ميزانية أداء فاشلة، وملفات PSD التي يكتبها PhotoCraft *"not yet reliably accepted by Photoshop"* ‏(#1281، #2469)‏.
- README يصرّح أن التطبيق *"not yet a Photoshop replacement for daily professional work"* (~25–35% من احتياجات المحترف اليومية عند الإطلاق، تحسّنت في ROADMAP إلى ~45% جاهزية)؛ FilmCraft يقيس نفسه ~86% عرض ميزات لكن ~56% "جاهز لعمل حقيقي" [م.ث].
- مختبرون أوائل رصدوا بطئاً ومشاكل في تحرير النصوص واختصارات لا تستجيب [م.ث].
- الدرس الأهم الذي صاغه المقال التحليلي [م.ث]: هناك ثلاثة أشياء مختلفة تُخلط معاً — شرح كيفية عمل برنامج، إعادة بناء ميزة، والاستبدال الموثوق لسير عمل احترافي. **‏"AI is doing the second faster than ever. But the third still takes time, real data, and testing under real conditions."** وبالصيغة المكثفة: *"the cost of attempting to replace software is falling fast, but the cost of proving it's reliable is still high."*
- ‏"627/628 بند قائمة مربوط بأوامر" يثبت **الاتصال** لا **صحة السلوك** — ولهذا توجد الـ oracles والـ floors وميزانيات الأداء كطبقات منفصلة.
- الجدل العام (HN، 246 تعليقاً): اتهامات "vibe coding"‏، وردود تفصّل: *"There's a difference between 'vibe coding' and incorporating AI into a software development process… They incorporated dozens of pre-existing Rust packages, made sure the apps ran on macOS, Windows and Linux and in modern web browsers. That's not vibe coding."* — الحكم الموضوعي هو بنية المستودع نفسها الموثقة أعلاه.

---

## 13. الخلاصات المستقطرة

1. **الجودة منتجٌ للاستحالة لا للانضباط.** كل قاعدة جوهرية لها بوابة آلية ترفض مخالفتها (clippy، xtask layers، CI، corpus floors، المترجم نفسه). الاعتماد على "انتباه" الوكيل وصفة للفشل عند 6,300 commits.
2. **‏"Everything is a command" هو حجر الزاوية.** تجريد واحد (أمر بـ id + JSON) يخدم في آن: الـ UI الرقيق، CLI‏، قناة التحكم، MCP‏، fuzzing الشامل، وعدّ التكافؤ. بدونه كان كل سطح سيحتاج تكاملاً خاصاً.
3. **الوثائق الحية المُقاسة هي نظام إدارة المشروع.** ‏gaps.md مرتب بالأثر + تقديرات معايرة + devlog مختصر = وكيل جديد (أو جلسة جديدة بلا سياق) يلتقط العمل خلال دقائق. الأرقام مولّدة آلياً لأن الرقم المكتوب يدوياً يكذب.
4. **الحلقة المغلقة تنهي الاعتماد على عين البشر لكل تغيير:** اختبار قبول أولاً ← تنفيذ ← قيادة التطبيق برمجياً ← لقطة شاشة ينظر إليها الوكيل ← توثيق. البشر يراجعون العينات والقرارات، لا كل سطر.
5. **التوازي يحتاج هندسة ملكيات:** worktree + target dir + مالك واحد لكل crate + وحدات جديدة بدل ملفات مشتركة + كتابة ذرّية + "لا تصلح ملفات غيرك". النزاعات تُمنع بنيوياً قبل أن تُحل.
6. **الاستقلالية مقيدة بقائمة تصعيد مغلقة:** الوكيل لا يتعطل ولا يتجرأ — يسأل في ست فئات محددة فقط، و*"when in doubt, leave it out"*.
7. **الـ Ratchets تحمي المكاسب:** حدود أرضية لا تنزل (parity/corpus/perf) + كل إصلاح انحدار معه اختبار + وثائق بختم مراجعة.
8. **وحدة القياس الجديدة: ساعات-الوكيل.** التقدير والتخطيط والمعايرة كلها بوحدة "وكيل واحد يعمل بتسلسل"، مع معرفة نسبة القابل للتوازي (~80%).
9. **الأمانة الرقمية جزء من النظام:** ‏"Why alpha" معلنة، والميزانيات الفاشلة تُعدّ، والجاهزية تُقدَّر بنطاقات — لا تسويق فوق الاختبار.
10. **السرعة لا تختصر إثبات الموثوقية** — وهذا ليس فشلاً للطريقة بل جوهرها: النظام صُمم لجعل الإثبات تراكمياً قابلاً للقياس بدل كونه حملة متأخرة.

---

## 14. الإسقاط على MiData

### 14.1 جدول الاستيراد المباشر

| آلية PhotoCraft | المقابل في MiData Native |
|---|---|
| ‏AGENTS.md يُقرأ أولاً | دستور MiData‏ (بالإنجليزية — لغة تنفيذ الوكلاء): الطبقات، Never-crash (بلا unwrap/expect/panic خارج الاختبارات — مفروض بـ clippy deny)، "المدخلات معادية"، كل إصلاح مع اختبار انحدار، قائمة التصعيد |
| ‏`cargo xtask layers` | ‏`cargo xtask check-layers` يفرض `domain ← store ← analytics/ai ← app` — تسرب المنطق (مرض النسخة القديمة: flattening وvalidation داخل controllers) يصبح مستحيلاً بنيوياً |
| Everything is a command | **سجل أوامر MiData في صلب النطاق من الطور 0–1**: كل عملية (تسجيل قياس، ولادة، استيراد، تعديل حقل، تقييم صيغة…) أمر بـ id + مخطط params موثّق + مسند enabled + ‏`Result<JSON>` + خطوة undo واحدة. تستدعيه: واجهة egui، وCLI (تشغيل دفعي/نسخ احتياطي)، ومساعد AI في الطور 5، وجسر الهاتف المستقبلي — **بلا إعادة كتابة** |
| ‏`state.rs` serde + ‏`ui.inspect` | حالة تطبيق MiData قابلة للتسلسل والفحص من اليوم الأول (درز جاهز لقناة تحكم JSON محلية في طور AI — loopback + token كما عندهم) |
| ‏gaps.md + ROADMAP.md + scorecard | ثلاث وثائق حية: قائمة عمل مرتبة (تنطلق من جرد الميزات في PROJECT_REPORT.md)، صفحة أرقام رئيسية (شاشات/أوامر/اختبارات/تغطية — مقاسة ومولّدة)، وdevlog مختصر بعد كل مهمة (حل انقطاع سياق الوكيل) |
| تقدير ساعات-الوكيل | تقدير أطوار MiData بـ"جلسات/ساعات وكيل" مع معايرة دورية من المنجز الفعلي |
| الاختبارات قبل الكود + Definition of Done | لكل مهمة معيار قبول مكتوب أولاً؛ وDoD: أمر مسجّل موثّق + اختبارات (بما فيها التراجع undo حيث ينطبق) + مربوط بالواجهة + CI أخضر + الوثائق محدّثة |
| ‏485 اختباراً في المشروع القديم | **oracles السلوك**: لا نترجم كودها (قرار مالك المشروع) بل نحوّل سيناريوهاتها إلى معايير قبول للنظام الجديد — نظير "PSD corpus" عندهم: الحقيقة المرجعية خارج الكود |
| offscreen snapshot + ‏"Look at the PNG" | ‏egui يرسم بلا نافذة — لقطات للشاشات الرئيسية في CI (حماية من انحدار الواجهة)؛ الوكيل ينظر إلى اللقطة قبل إعلان إنجاز أي شاشة |
| ‏panic_hunt | اختبار fuzz لسجل أوامر MiData: كل أمر بمدخلات معادية (قيم سالبة، نصوص فارغة، تواريخ مستحيلة، ids غير موجودة) → ‏Err لا panic |
| Ratchets (FLOOR لا ينزل) | عدادات أحادية: عدد الأوامر المغطاة باختبارات، عدد الشاشات بلقطات مرجعية، تغطية i18n‏ (نمط TSV + ‏`xtask i18n-coverage` لـ EN/FR — الآلية مجرّبة حرفياً) |
| Worktrees + ملكية crate | تُوثّق في الدستور منذ البداية (وإن بدأ التنفيذ بوكيل واحد): وحدة جديدة لكل منطقة أوامر، ملفات مشتركة بتعديلات جراحية، كتابة ذرّية للملفات الحساسة |
| قائمة التصعيد | حذف/تعديل بيانات، تغيير مخطط القاعدة (migration جديد)، إضافة تبعية (license + صيانة)، أي نشر — يسأل المالك. ما عداها الوكيل يحسم |
| المزيج متعدد النماذج | مرونة اختيار النموذج لكل نوع مهمة (تخطيط/تنفيذ/مراجعة) — لا التزام بنموذج واحد |
| Clean-room | لا ينطبق قانونياً (نحن نملك الأصل) لكن **روحياً معتمد**: PROJECT_REPORT.md هو مواصفة السلوك، ولا يُنسخ كود قديم بعيوبه |

### 14.2 "موجّه للوكلاء" كمبدأ من الدرجة الأولى (قرار المالك)

> **الموقف المعتمد لمشروعنا:** *"البرامج في هذا العصر يجب أن تكون موجّهة للوكلاء بقدر ما هي موجّهة للبشر — المستخدمون أصبحوا بشراً ووكلاء، وليس بشراً فقط."* — مالك المشروع، 2026-10-27

هذا يرفع نمط ArtCraft من "تكتيك تطوير" إلى **متطلب منتج** في MiData‏، بالنتائج المعمارية التالية:

1. **محرك أوامر من الطور 0** (وليس ترفاً في طور AI): ‏`midata-core` يعرّف Session + Command Registry‏؛ كل سلوك مستخدم = أمر. الـ UI طبقة عرض رقيقة.
2. **كل حالة مفحوصة:** حالة التطبيق serde ‏(مثل `state.rs`) — أي مستهلك آلي (CLI‏، قناة تحكم، MCP‏، جسر هاتف مستقبلي) يقرأ ويكتب عبر نفس الواجهة.
3. **كل أمر ذاتي التوثيق:** ‏id + وصف + مخطط params + نوع النتيجة JSON — هذا ما يقرؤه الوكيل (عبر `commands list`) لا الكود المصدري.
4. **CLI مواطن أول:** ‏`midata-cli` يستدعي نفس الأوامر (استيراد دفعي، تصدير، نسخ احتياطي، تقارير) — يفيد المزارع التقني والوكيل معاً، ويكلف هامشياً إذا وُجد السجل من اليوم الأول.
5. **قناة تحكم محلية + MCP** تُبنى في طور AI فوق نفس البروتوكول — المساعد الداخلي والمساعدات الخارجية (Claude Desktop‏، Cursor…) سواء.
6. **سجل التدقيق = قابلية ملاحظة الوكلاء:** كل تنفيذ أمر يُسجَّل (من، متى، ماذا تغيّر) — للبشر تدقيق وللبشر ثقة، وللوكلاء سياق.
7. **‏AGENTS.md جزء من المنتج:** المستودع نفسه يُستهلك من وكلاء (وكيل المالك اليوم، وكلاء المساهمين غداً) — التوثيق التشغيلي أولوية إصدار لا ترفيق لاحق.

### 14.3 ما لا نحتاجه منهم

- قواعد clean-room القانونية (الأصل ملكنا — نكتفي بروح "البناء من المواصفة لا من الكود").
- WASM/Web و"never break wasm" ‏(v1 سطح مكتب أصيل فقط؛ الوصول عبر الويب/الهاتف مؤجل وموثّق كخيار مستقبلي).
- البنية متعددة التطبيقات (`craftrules`/`craft-libs`/`craft-fonts`/launcher) — مستودع واحد يكفي، مع إبقاء المعايير في مجلد `docs/` قابل للترقية لاحقاً.
- حجم التوازي الهائل (311 فرعاً، tracks‏) — يكفي وكيل واحد أو اثنان بقواعد الملكية نفسها.
- آليات الاعتماد المجتمعي (contributors.json/people.toml/About credits) — تُستورد فقط إذا فُتح المستودع للعموم.
- MCP وخادم التحكم في الإصدار الأول (لكن **الدرز المعماري موجود من اليوم الأول** — التنفيذ في طور AI).

### 14.4 معايرة الحجم (حتى لا نخدع أنفسنا)

PhotoCraft أكبر من MiData بمقدار جوهري (معالجات GPU‏، محرك فرش، أمانة PSD‏، 628 بند قائمة، 5 منصات…). لكن المنهجية **تتحجم نزولاً بأناقة**: الحد الأدنى الفعّال لمطوّر واحد + وكيل واحد هو:
**(1)‏ AGENTS.md · (2) قائمة عمل مرتبة (gaps-style) · (3) بوابة إنهاء واحدة (`cargo xtask ci` تجمع fmt+clippy+test+layers) · (4) devlog · (5) سجل أوامر + اختبارات أوامر · (6) لقطات offscreen للشاشات**.
هذه الستة تمنح ~80% من أثر المنهجية بتكلفة إعداد تُقاس بأيام لا أسابيع — وستكون العمود الفقري لوثيقة `REBUILD_PLAN.md` القادمة.

---

## 15. المصادر

**مصادر أولية (قُرئت مباشرة أثناء التحقيق):**

- المستودع: ‏https://github.com/storytold/photocraft ‏(README، البيانات الوصفية عبر GitHub API، سجل commits)
- ‏AGENTS.md الكامل: ‏https://github.com/storytold/photocraft/blob/main/AGENTS.md
- قواعد المساهمة وchecklist "إضافة أمر": ‏https://github.com/storytold/photocraft/blob/main/docs/contributing.md
- بروتوكول التحكم: ‏https://github.com/storytold/photocraft/blob/main/docs/control-protocol.md
- قائمة الفجوات المرتبة: ‏https://github.com/storytold/photocraft/blob/main/docs/gaps.md
- خارطة الطريق المُقاسة: ‏https://github.com/storytold/photocraft/blob/main/ROADMAP.md
- قائمة التطبيقات الثلاثة عشر: ‏https://getartcraft.com/apps
- نقاش Hacker News ‏(246 تعليقاً): ‏https://news.ycombinator.com/item?id=49958850

**مصادر ثانوية:**

- تحليل العملية والخمس خطوات وأرقام co-authorship: ‏https://greennode.ai/blog/what-is-artcraft ‏(2026-10-12)
- تغطية صحفية: ‏trendingtopics.eu ‏(2026-10-12)، cined.com ‏(2026-10-11)، dpreview forums ‏(2026-10-08)، tedium.co ‏(2026-10-08)

**وثيقة ذات صلة داخل مساحة العمل:** ‏`uploads/PROJECT_REPORT.md` — مواصفة سلوك MiData القديمة (المرجع الوظيفي لإعادة البناء).

---

*نهاية التحقيق. الخطوة التالية المتفق عليها: العودة إلى مشروعنا وصياغة `REBUILD_PLAN.md` + ‏`AGENTS.md` الخاص بـ MiData على ضوء هذه الخلاصات — عند إذن المالك.*
