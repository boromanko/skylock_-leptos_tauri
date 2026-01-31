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
                    <a href="#list-item" class="gallery__nav-link">"List Item Combos"</a>
                </nav>
            </aside>
        </div>
    }
}
