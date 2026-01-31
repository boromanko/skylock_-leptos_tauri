use leptos::prelude::*;

#[component]
pub fn GalleryScreen() -> impl IntoView {
    let color_swatches = [
        ("Surface Base", "--surface-base"),
        ("Surface Raised", "--surface-raised"),
        ("Text Primary", "--text-primary"),
        ("Text Secondary", "--text-secondary"),
        ("Accent Primary", "--accent-primary"),
        ("Danger", "--text-danger"),
    ];

    let semantic_colors = [
        ("Text Primary", "--text-primary"),
        ("Text Secondary", "--text-secondary"),
        ("Text Muted", "--text-muted"),
        ("Text Disabled", "--text-disabled"),
        ("Text Danger", "--text-danger"),
        ("Text On Primary", "--text-on-primary"),
        ("Icon Primary", "--icon-primary"),
        ("Icon Secondary", "--icon-secondary"),
        ("Icon Disabled", "--icon-disabled"),
        ("Surface Base", "--surface-base"),
        ("Surface Muted", "--surface-muted"),
        ("Surface Raised", "--surface-raised"),
        ("Border Subtle", "--border-subtle"),
        ("Border Focus", "--border-focus"),
        ("Action Default", "--action-default"),
        ("Action Hover", "--action-hover"),
        ("Action Focus", "--action-focus"),
        ("Action Pressed", "--action-pressed"),
        ("Action Disabled", "--action-disabled"),
        ("Action Primary", "--action-primary"),
        ("Action Primary Hover", "--action-primary-hover"),
        ("Action Primary Focus", "--action-primary-focus"),
        ("Action Primary Pressed", "--action-primary-pressed"),
        ("Action Primary Disabled", "--action-primary-disabled"),
        ("Accent Primary", "--accent-primary"),
        ("Accent Primary Hover", "--accent-primary-hover"),
        ("Accent Danger", "--accent-danger"),
        ("Accent Danger Hover", "--accent-danger-hover"),
        ("Switch Track On", "--switch-track-on"),
        ("Switch Track Off", "--switch-track-off"),
        ("Switch Track Disabled", "--switch-track-disabled"),
        ("Switch Thumb", "--switch-thumb"),
        ("Switch Thumb Disabled", "--switch-thumb-disabled"),
    ];

    let semantic_texts = [
        ("Text Primary", "--text-primary", None),
        ("Text Secondary", "--text-secondary", None),
        ("Text Muted", "--text-muted", None),
        ("Text Disabled", "--text-disabled", None),
        ("Text Danger", "--text-danger", None),
        ("Text On Primary", "--text-on-primary", Some("--action-primary")),
    ];

    view! {
        <div class="gallery">
            <div class="gallery__content">
                <section id="colors" class="gallery__section">
                    <h2 class="gallery__title">"Colors"</h2>
                    <div class="gallery__swatches">
                        {color_swatches
                            .iter()
                            .map(|(label, var_name)| {
                                view! {
                                    <div class="swatch">
                                        <div
                                            class="swatch__chip"
                                            style=format!("background: var({});", var_name)
                                        />
                                        <div class="swatch__meta">
                                            <div class="swatch__label">{*label}</div>
                                            <div class="swatch__var">{*var_name}</div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </section>

                <section id="semantic-colors" class="gallery__section">
                    <h2 class="gallery__title">"Semantic Colors"</h2>
                    <div class="gallery__swatches">
                        {semantic_colors
                            .iter()
                            .map(|(label, var_name)| {
                                view! {
                                    <div class="swatch">
                                        <div
                                            class="swatch__chip"
                                            style=format!("background: var({});", var_name)
                                        />
                                        <div class="swatch__meta">
                                            <div class="swatch__label">{*label}</div>
                                            <div class="swatch__var">{*var_name}</div>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </section>

                <section id="semantic-text" class="gallery__section">
                    <h2 class="gallery__title">"Semantic Text"</h2>
                    <div class="gallery__text-samples">
                        {semantic_texts
                            .iter()
                            .map(|(label, color_var, bg_var)| {
                                let style = match bg_var {
                                    Some(bg) => {
                                        format!("color: var({}); background: var({});", color_var, bg)
                                    }
                                    None => format!("color: var({});", color_var),
                                };
                                view! {
                                    <div class="text-sample" style=style>
                                        <div class="text-sample__label">{*label}</div>
                                        <div class="text-sample__var">{*color_var}</div>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </section>

                <section id="typography" class="gallery__section">
                    <h2 class="gallery__title">"Typography"</h2>
                    <div class="gallery__stack">
                        <div class="text-16-600">"Text 16 / 600"</div>
                        <div class="text-12-400 text-muted">"Text 12 / 400"</div>
                    </div>
                </section>

                <section id="button" class="gallery__section">
                    <h2 class="gallery__title">"Button"</h2>
                    <div class="gallery__stack">
                        <button class="button pressable">
                            <span class="button__label">"Default"</span>
                        </button>

                        <button class="button button--primary pressable">
                            <span class="button__icon centered">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-download);"></span>
                            </span>
                            <span class="button__label">"Primary"</span>
                        </button>

                        <button class="button button--danger pressable">
                            <span class="button__icon centered">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-delete);"></span>
                            </span>
                            <span class="button__label">"Danger"</span>
                        </button>

                        <button class="button button--disabled" disabled>
                            <span class="button__label">"Disabled"</span>
                        </button>

                        <button class="button button--primary pressable">
                            <span class="button__icon centered">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-download);"></span>
                            </span>
                        </button>
                    </div>
                </section>

                <section id="editable-field" class="gallery__section">
                    <h2 class="gallery__title">"Editable Field"</h2>
                    <div class="gallery__stack">
                        <div class="editable-field editable-field--start">
                            <div class="editable-field__label text-16-500">"Latitude"</div>
                            <div class="editable-field__value text-48-500">
                                <span class="editable-field__text">"49.8397"</span>
                                <span class="editable-field__caret"></span>
                            </div>
                        </div>

                        <div class="editable-field editable-field--start editable-field--error">
                            <div class="editable-field__label text-16-500">"Longitude"</div>
                            <div class="editable-field__value text-48-500">
                                <span class="editable-field__text">"181.0000"</span>
                                <span class="editable-field__caret"></span>
                            </div>
                        </div>

                        <div class="editable-field editable-field--start">
                            <div class="editable-field__label text-16-500">"Preview"</div>
                            <div class="editable-field__value text-48-500">
                                <span class="editable-field__text">"1234"</span>
                                <span class="editable-field__caret"></span>
                            </div>
                        </div>

                        <div class="editable-field editable-field--start editable-field--disabled">
                            <div class="editable-field__label text-16-500">"Disabled"</div>
                            <div class="editable-field__value text-48-500">
                                <span class="editable-field__text">"---"</span>
                                <span class="editable-field__caret"></span>
                            </div>
                        </div>
                    </div>
                </section>

                <section id="overlay" class="gallery__section">
                    <h2 class="gallery__title">"Overlay / Modal"</h2>
                    <div class="overlay-demo">
                        <div class="overlay overlay--demo">
                            <div class="overlay__panel">
                                <div class="overlay__title text-16-600">"Confirm delete"</div>
                                <div class="overlay__body text-12-400 text-muted">
                                    "Are you sure you want to remove this item?"
                                </div>
                                <div class="overlay__actions">
                                    <button class="button pressable">"Cancel"</button>
                                    <button class="button button--primary pressable">"Delete"</button>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>

                <section id="keyboard" class="gallery__section">
                    <h2 class="gallery__title">"Keyboard"</h2>
                    <div class="gallery__stack">
                        <div class="keyboard keyboard--text">
                            <button class="button pressable keyboard__key">"1"</button>
                            <button class="button pressable keyboard__key">"2"</button>
                            <button class="button pressable keyboard__key">"3"</button>
                            <button class="button pressable keyboard__key">"4"</button>
                            <button class="button pressable keyboard__key">"5"</button>
                            <button class="button pressable keyboard__key">"6"</button>
                            <button class="button pressable keyboard__key">"7"</button>
                            <button class="button pressable keyboard__key">"8"</button>
                            <button class="button pressable keyboard__key">"9"</button>
                            <button class="button pressable keyboard__key">"0"</button>
                            <button class="button pressable keyboard__key">"-"</button>
                            <button class="button pressable keyboard__key">"="</button>

                            <button class="button pressable keyboard__key">"й"</button>
                            <button class="button pressable keyboard__key">"ц"</button>
                            <button class="button pressable keyboard__key">"у"</button>
                            <button class="button pressable keyboard__key">"к"</button>
                            <button class="button pressable keyboard__key">"е"</button>
                            <button class="button pressable keyboard__key">"н"</button>
                            <button class="button pressable keyboard__key">"г"</button>
                            <button class="button pressable keyboard__key">"ш"</button>
                            <button class="button pressable keyboard__key">"щ"</button>
                            <button class="button pressable keyboard__key">"з"</button>
                            <button class="button pressable keyboard__key">"х"</button>
                            <button class="button pressable keyboard__key">"ї"</button>

                            <button class="button pressable keyboard__key">"ф"</button>
                            <button class="button pressable keyboard__key">"і"</button>
                            <button class="button pressable keyboard__key">"в"</button>
                            <button class="button pressable keyboard__key">"а"</button>
                            <button class="button pressable keyboard__key">"п"</button>
                            <button class="button pressable keyboard__key">"р"</button>
                            <button class="button pressable keyboard__key">"о"</button>
                            <button class="button pressable keyboard__key">"л"</button>
                            <button class="button pressable keyboard__key">"д"</button>
                            <button class="button pressable keyboard__key">"ж"</button>
                            <button class="button pressable keyboard__key">"є"</button>
                            <button class="button pressable keyboard__key">"ґ"</button>

                            <button class="button pressable keyboard__key">"\\"</button>
                            <button class="button pressable keyboard__key">"я"</button>
                            <button class="button pressable keyboard__key">"ч"</button>
                            <button class="button pressable keyboard__key">"с"</button>
                            <button class="button pressable keyboard__key">"м"</button>
                            <button class="button pressable keyboard__key">"и"</button>
                            <button class="button pressable keyboard__key">"т"</button>
                            <button class="button pressable keyboard__key">"ь"</button>
                            <button class="button pressable keyboard__key">"б"</button>
                            <button class="button pressable keyboard__key">"ю"</button>
                            <button class="button pressable keyboard__key">"."</button>
                            <button class="button pressable keyboard__key">"/"</button>

                            <button class="button pressable keyboard__key keyboard__key--span-2">"Shift"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-6">"Space"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-backspace);"></span>
                            </button>
                            <button class="button button--primary keyboard__key pressable keyboard__key--span-2">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-check-circle);"></span>
                            </button>
                        </div>

                        <div class="keyboard keyboard--text keyboard--shift">
                            <button class="button pressable keyboard__key">"!"</button>
                            <button class="button pressable keyboard__key">"@"</button>
                            <button class="button pressable keyboard__key">"№"</button>
                            <button class="button pressable keyboard__key">";"</button>
                            <button class="button pressable keyboard__key">"%"</button>
                            <button class="button pressable keyboard__key">":"</button>
                            <button class="button pressable keyboard__key">"?"</button>
                            <button class="button pressable keyboard__key">"*"</button>
                            <button class="button pressable keyboard__key">"("</button>
                            <button class="button pressable keyboard__key">")"</button>
                            <button class="button pressable keyboard__key">"_"</button>
                            <button class="button pressable keyboard__key">"+"</button>

                            <button class="button pressable keyboard__key">"Й"</button>
                            <button class="button pressable keyboard__key">"Ц"</button>
                            <button class="button pressable keyboard__key">"У"</button>
                            <button class="button pressable keyboard__key">"К"</button>
                            <button class="button pressable keyboard__key">"Е"</button>
                            <button class="button pressable keyboard__key">"Н"</button>
                            <button class="button pressable keyboard__key">"Г"</button>
                            <button class="button pressable keyboard__key">"Ш"</button>
                            <button class="button pressable keyboard__key">"Щ"</button>
                            <button class="button pressable keyboard__key">"З"</button>
                            <button class="button pressable keyboard__key">"Х"</button>
                            <button class="button pressable keyboard__key">"Ї"</button>

                            <button class="button pressable keyboard__key">"Ф"</button>
                            <button class="button pressable keyboard__key">"І"</button>
                            <button class="button pressable keyboard__key">"В"</button>
                            <button class="button pressable keyboard__key">"А"</button>
                            <button class="button pressable keyboard__key">"П"</button>
                            <button class="button pressable keyboard__key">"Р"</button>
                            <button class="button pressable keyboard__key">"О"</button>
                            <button class="button pressable keyboard__key">"Л"</button>
                            <button class="button pressable keyboard__key">"Д"</button>
                            <button class="button pressable keyboard__key">"Ж"</button>
                            <button class="button pressable keyboard__key">"Є"</button>
                            <button class="button pressable keyboard__key">"Ґ"</button>

                            <button class="button pressable keyboard__key">"\\"</button>
                            <button class="button pressable keyboard__key">"Я"</button>
                            <button class="button pressable keyboard__key">"Ч"</button>
                            <button class="button pressable keyboard__key">"С"</button>
                            <button class="button pressable keyboard__key">"М"</button>
                            <button class="button pressable keyboard__key">"И"</button>
                            <button class="button pressable keyboard__key">"Т"</button>
                            <button class="button pressable keyboard__key">"Ь"</button>
                            <button class="button pressable keyboard__key">"Б"</button>
                            <button class="button pressable keyboard__key">"Ю"</button>
                            <button class="button pressable keyboard__key">"."</button>
                            <button class="button pressable keyboard__key">"/"</button>

                            <button class="button pressable keyboard__key keyboard__key--span-2">"Shift"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-6">"Space"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-backspace);"></span>
                            </button>
                            <button class="button button--primary keyboard__key pressable keyboard__key--span-2">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-check-circle);"></span>
                            </button>
                        </div>

                        <div class="keyboard keyboard--numeric">
                            <button class="button pressable keyboard__key keyboard__key--span-2">"1"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"2"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"3"</button>

                            <button class="button pressable keyboard__key keyboard__key--span-2">"4"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"5"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"6"</button>

                            <button class="button pressable keyboard__key keyboard__key--span-2">"7"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"8"</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"9"</button>

                            <button class="button button--disabled keyboard__key" disabled>"-"</button>
                            <button class="button button--disabled keyboard__key" disabled>"."</button>
                            <button class="button pressable keyboard__key keyboard__key--span-2">"0"</button>
                            <button class="button pressable keyboard__key">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-backspace);"></span>
                            </button>
                            <button class="button button--primary pressable keyboard__key">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-check-circle);"></span>
                            </button>
                        </div>
                    </div>
                </section>

                <section id="toast" class="gallery__section">
                    <h2 class="gallery__title">"Toast"</h2>
                    <div class="gallery__stack">
                        <div class="toast">
                            <div class="toast__content">
                                <div class="toast__title text-14-600">"Інформаційне повідомлення"</div>
                            </div>
                        </div>

                        <div class="toast toast--success">
                            <span class="toast__icon centered">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-check-circle);"></span>
                            </span>
                            <div class="toast__content">
                                <div class="toast__title text-14-600">"Успіх"</div>
                            </div>
                        </div>

                        <div class="toast toast--warning">
                            <span class="toast__icon centered">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-warning);"></span>
                            </span>
                            <div class="toast__content">
                                <div class="toast__title text-14-600">"Попередження"</div>
                            </div>
                        </div>

                        <div class="toast toast--error">
                            <span class="toast__icon centered">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-error);"></span>
                            </span>
                            <div class="toast__content">
                                <div class="toast__title text-14-600">"Помилка"</div>
                            </div>
                        </div>

                        <div class="toast toast--target">
                            <span class="toast__icon centered">
                                <img class="toast__icon-image" src="/public/images/target-focus.svg" alt="Target acquired" />
                            </span>
                            <div class="toast__content">
                                <div class="toast__title text-16-600">"Ціль захоплено"</div>
                            </div>
                        </div>
                    </div>
                </section>

                <section id="list-item" class="gallery__section">
                    <h2 class="gallery__title">"List Item Combinations"</h2>
                    <div class="gallery__stack">
                        <div class="list-item pressable list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-settings);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Settings"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Subtitle"</div>
                            </div>
                            <div class="list-item__trailing">
                                <div class="list-item__trailing-main">
                                    <div class="radio radio--checked">
                                        <span class="icon icon--28 icon--filled icon--inherit" style="--icon-glyph: var(--icon-radio-button-checked);"></span>
                                    </div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--primary list-item--with-leading" tabindex="0">
                            <div class="list-item__leading list-item__leading--primary">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-download);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Update"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Available"</div>
                            </div>
                            <div class="list-item__trailing list-item__trailing--no-chevron">
                                <div class="list-item__trailing-main">
                                    <div class="toggle toggle--on"></div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--danger list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-delete);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Delete"</div>
                                <div class="list-item__subtitle text-12-400">"Remove data"</div>
                            </div>
                            <div class="list-item__trailing">
                                <div class="list-item__trailing-main">
                                    <div class="checkbox">
                                        <span class="icon icon--28 icon--filled icon--inherit" style="--icon-glyph: var(--icon-checkbox-unchecked);"></span>
                                    </div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--with-leading list-item--active" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-settings);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Active Row"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"With chevron"</div>
                            </div>
                            <div class="list-item__trailing">
                                <div class="list-item__trailing-main">
                                    <div class="radio">
                                        <span class="icon icon--28 icon--filled icon--inherit" style="--icon-glyph: var(--icon-radio-button-unchecked);"></span>
                                    </div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-settings);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Toggle Off"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Chevron hidden"</div>
                            </div>
                            <div class="list-item__trailing list-item__trailing--no-chevron">
                                <div class="list-item__trailing-main">
                                    <div class="toggle"></div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-settings);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Checkbox Checked"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Chevron visible"</div>
                            </div>
                            <div class="list-item__trailing">
                                <div class="list-item__trailing-main">
                                    <div class="checkbox checkbox--checked">
                                        <span class="icon icon--28 icon--filled icon--inherit" style="--icon-glyph: var(--icon-checkbox-checked);"></span>
                                    </div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-settings);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Radio Unchecked"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Chevron hidden"</div>
                            </div>
                            <div class="list-item__trailing list-item__trailing--no-chevron">
                                <div class="list-item__trailing-main">
                                    <div class="radio">
                                        <span class="icon icon--28 icon--filled icon--inherit" style="--icon-glyph: var(--icon-radio-button-unchecked);"></span>
                                    </div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--primary list-item--with-leading" tabindex="0">
                            <div class="list-item__leading list-item__leading--primary">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-download);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Primary Row"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Chevron visible"</div>
                            </div>
                            <div class="list-item__trailing">
                                <div class="list-item__trailing-main">
                                    <div class="checkbox checkbox--disabled">
                                        <span class="icon icon--28 icon--filled icon--inherit" style="--icon-glyph: var(--icon-checkbox-unchecked);"></span>
                                    </div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable list-item--danger list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-delete);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Danger Row"</div>
                                <div class="list-item__subtitle text-12-400">"Chevron hidden"</div>
                            </div>
                            <div class="list-item__trailing list-item__trailing--no-chevron">
                                <div class="list-item__trailing-main">
                                    <div class="toggle toggle--disabled"></div>
                                </div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>

                        <div class="list-item pressable" tabindex="0">
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"No Leading"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"No trailing"</div>
                            </div>
                            <div class="list-item__trailing">
                                <div class="list-item__trailing-main"></div>
                                <div class="list-item__trailing-arrow">
                                    <span class="icon icon--24 icon--filled icon--inherit" style="--icon-glyph: var(--icon-chevron-right);"></span>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </div>

            <aside class="gallery__sidebar">
                <h3 class="gallery__sidebar-title">"Gallery"</h3>
                <nav class="gallery__nav">
                    <a href="#colors" class="gallery__nav-link">"Colors"</a>
                    <a href="#semantic-colors" class="gallery__nav-link">"Semantic Colors"</a>
                    <a href="#semantic-text" class="gallery__nav-link">"Semantic Text"</a>
                    <a href="#typography" class="gallery__nav-link">"Typography"</a>
                    <a href="#button" class="gallery__nav-link">"Button"</a>
                    <a href="#editable-field" class="gallery__nav-link">"Editable Field"</a>
                    <a href="#overlay" class="gallery__nav-link">"Overlay"</a>
                    <a href="#keyboard" class="gallery__nav-link">"Keyboard"</a>
                    <a href="#toast" class="gallery__nav-link">"Toast"</a>
                    <a href="#list-item" class="gallery__nav-link">"List Item Combos"</a>
                </nav>
            </aside>
        </div>
    }
}
