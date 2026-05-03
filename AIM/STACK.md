# AIM — STACK rule (постоянное правило ядра)

**Дата установки:** 2026-05-04
**Источник:** прямая директива пользователя ("delai vse na Rust",
"frontend na Phoenix", "prodoljit razvitie AIM — na Rust i Phoenix").

---

## Единственное технологическое правило

**Всё, что разрабатывается в AIM, пишется на:**

- **Backend / CLI / агенты / алгоритмы / БД-доступ / системные сервисы → Rust**
  (workspace `AIM/rust-core/`, новые крейты в `crates/aim-*`)
- **Frontend / dashboards / interactive UI → Phoenix LiveView**
  (Elixir, по паттерну Ze/BioSense/FCLC: `mix release` → systemd
  `/opt/aim-*/`, native, без Docker runtime)

**Никаких новых модулей на Python без явной необходимости.**

---

## Что считается "явной необходимостью" для Python

Только если задача требует библиотеки, для которой **нет зрелого Rust
аналога**, и переписывать её = огромная работа без пропорциональной
выгоды. На 2026-05-04 список:

| Задача | Python-зависимость | Почему нет Rust |
|--------|--------------------|-----------------|
| OCR | `rapidocr_onnxruntime`, `tesseract` | tesseract-rs незрелый, ONNX Runtime Rust есть но без полного OCR pipeline |
| PDF text extraction | `pymupdf`, `pdfplumber` | `lopdf` / `pdfium-render` ограничены |
| WhatsApp pipeline | существующий `agents/intake.py` | работает, шим через subprocess из Rust |
| TTS офлайн (v7.2) | `xtts-v2`, `pyttsx3`, `espeak-ng` | нет нативного Rust voice cloning |
| Talking-head офлайн (v7.2) | `SadTalker`, `Wav2Lip` | PyTorch-only, нет Rust-портов |
| 3D mesh офлайн (v7.2) | `Hunyuan3D`, `TripoSR`, Microsoft `Trellis` | diffusion-based, PyTorch-only |
| 3D scripting (v7.2) | Blender `bpy` | Blender Python — единственный API; вызов `blender --background --python` |
| Молекулярная визуализация (v7.2) | `RDKit`, `py3Dmol`, PyMOL | хемоинформатика — только Python; рендерим в GLB → Three.js |
| DICOM 3D (v7.2) | 3D Slicer (`vtk`, `pydicom`) | медицинская визуализация — Python экосистема |

**Все эти Python-блоки вызываются из Rust через subprocess или unix
socket.** Они не получают новой функциональности — только сохранение
существующего. Любые их расширения — пишем на Rust обёртку и
постепенно вытесняем Python.

---

## Что считается "разработкой в AIM"

- Новые модули в `AI/ai/` → пишутся как новые Rust crates в
  `rust-core/crates/aim-ai-*/` (НЕ как `.py` файлы)
- Новые модули в `agents/` → как Rust crates `crates/aim-agent-*/`
- Новые web-страницы / dashboards → как LiveView в `aim-web/lib/`
  (НЕ как новые `aim_gui.py` дополнения, НЕ как FastAPI endpoints
  в `web/api.py`)
- Новые CLI-команды → как subcommands в `aim` clap CLI (НЕ как новые
  argparse pieces в `aim_cli.py`)
- Новые тесты → Rust `#[cfg(test)]` или Phoenix `mix test`. pytest
  только для shimming Python legacy.

---

## Что НЕ нужно переписывать прямо сейчас

Существующий Python AIM **продолжает работать**, его не выкидываем.
Замена идёт по очереди — одно ядро за раз — с паритетным test suite.
Удаление Python-предка только когда Rust-замена покрывает 100% и
проходит eval-harness.

Порядок миграции зафиксирован в `MIGRATION_RUST_PHOENIX.md`. Текущий
sprint — Phase 1 (Hive layer на Rust).

---

## Соблюдение

Это правило применяется **на каждой сессии разработки AIM** автоматически.
Если AI-ассистент или человек собирается написать `*.py` в AIM/ — должна
сработать проверка: "это Phase X миграции, или это shim для существующей
OCR/PDF-зависимости?". Если ни то ни другое — правило нарушено.

При сомнении — спросить пользователя. Не писать Python "потому что
быстрее".

---

**Это файл ядра.** Он не генерируется из CONCEPT.md; он сам — правило.
В public git не загружается (через `.gitignore` core .md ruling).
