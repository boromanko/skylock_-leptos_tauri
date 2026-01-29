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
                                            <div class="swatch__label">{label}</div>
                                            <div class="swatch__var">{var_name}</div>
                                        </div>
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
                    <h2 class="gallery__title">"List Item"</h2>
                    <div class="gallery__stack">
                        <div class="list-item list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24" style="--icon-glyph: var(--icon-settings);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Settings"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Subtitle"</div>
                            </div>
                            <div class="list-item__trailing">
                                <span class="icon icon--28" style="--icon-glyph: var(--icon-arrow-right);"></span>
                            </div>
                        </div>

                        <div class="list-item list-item--primary list-item--with-leading" tabindex="0">
                            <div class="list-item__leading list-item__leading--primary">
                                <span class="icon icon--24 icon--filled" style="--icon-glyph: var(--icon-download);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Update"</div>
                                <div class="list-item__subtitle text-12-400 text-muted">"Available"</div>
                            </div>
                            <div class="list-item__trailing">
                                <span class="icon icon--28" style="--icon-glyph: var(--icon-arrow-right);"></span>
                            </div>
                        </div>

                        <div class="list-item list-item--danger list-item--with-leading" tabindex="0">
                            <div class="list-item__leading">
                                <span class="icon icon--24" style="--icon-glyph: var(--icon-delete);"></span>
                            </div>
                            <div class="list-item__content">
                                <div class="list-item__title text-16-600">"Delete"</div>
                                <div class="list-item__subtitle text-12-400">"Remove data"</div>
                            </div>
                            <div class="list-item__trailing">
                                <span class="icon icon--28" style="--icon-glyph: var(--icon-arrow-right);"></span>
                            </div>
                        </div>
                    </div>
                </section>
            </div>

            <aside class="gallery__sidebar">
                <h3 class="gallery__sidebar-title">"Gallery"</h3>
                <nav class="gallery__nav">
                    <a href="#colors" class="gallery__nav-link">"Colors"</a>
                    <a href="#typography" class="gallery__nav-link">"Typography"</a>
                    <a href="#list-item" class="gallery__nav-link">"List Item"</a>
                </nav>
            </aside>
        </div>
    }
}
