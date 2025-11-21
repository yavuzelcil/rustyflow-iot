use leptos::*;
use std::time::Duration;
mod api;
mod components;

use components::sensor_card::SensorCard;

/// Ana dashboard component'i
/// 
/// Bu component sensör verilerini API'den çeker ve ekranda gösterir.
/// Her 2 saniyede bir otomatik olarak verileri yeniler.
#[component]
fn App() -> impl IntoView {
    // API'den sensör verilerini çekmek için signal
    let (sensor_data, set_sensor_data) = create_signal(Vec::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // API'den veri çekme fonksiyonu
    let fetch_sensors = move || {
        spawn_local(async move {
            match api::fetch_sensor_data().await {
                Ok(data) => {
                    set_sensor_data.set(data);
                    set_loading.set(false);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_loading.set(false);
                }
            }
        });
    };

    // Component yüklendiğinde veri çek
    create_effect(move |_| {
        fetch_sensors();
    });

    // Her 2 saniyede bir otomatik yenile
    create_effect(move |_| {
        set_interval(
            move || fetch_sensors(),
            Duration::from_secs(2),
        );
    });

    view! {
        <div class="dashboard">
            <div class="dashboard-header">
                <h1>"🦀 RustyFlow IoT Dashboard"</h1>
                <p>"Real-time sensor monitoring with Rust + Leptos + WASM"</p>
            </div>

            {move || {
                if loading.get() && sensor_data.get().is_empty() {
                    view! {
                        <div class="loading">"Loading sensor data..."</div>
                    }.into_view()
                } else if let Some(err) = error.get() {
                    view! {
                        <div class="error">
                            <strong>"Error: "</strong>
                            {err}
                        </div>
                    }.into_view()
                } else {
                    view! {
                        <div class="sensor-grid">
                            <For
                                each=move || sensor_data.get()
                                key=|sensor| format!("{}-{}", sensor.device_id, sensor.sensor_type)
                                children=move |sensor| {
                                    view! {
                                        <SensorCard sensor=sensor/>
                                    }
                                }
                            />
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}

fn main() {
    // Panic mesajlarını browser console'a yazdır
    console_error_panic_hook::set_once();
    
    // Leptos uygulamasını başlat
    mount_to_body(|| view! { <App/> })
}
