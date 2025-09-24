use leptos::prelude::*;

#[server(SayHello)]
pub async fn say_hello(num: i32) -> Result<String, ServerFnError> {
    Ok(format!("Hello from the API!!! I got {num}"))
}

#[component]
pub fn ShowDataFromApi() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    let counter = RwSignal::new(0);

    let on_click = move |_| {
        leptos::task::spawn_local(async move {
            let api_said = say_hello(counter.get()).await.unwrap();
            value.set(api_said);
            counter.update(|v| *v += 1);
        });
    };

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gray-100 p-4">
            <div class="max-w-2xl w-full mx-auto p-6 bg-white rounded-xl shadow-lg">
                <div class="text-center mb-8">
                    <h2 class="text-2xl font-bold text-gray-800 mb-2">API Demo</h2>
                    <p class="text-gray-600">Click the button below to interact with the server function</p>
                </div>

                <div class="flex flex-col items-center justify-center space-y-6">
                    <button
                        class="bg-blue-600 hover:bg-blue-700 text-white font-bold py-3 px-6 rounded-lg shadow-md hover:shadow-lg transition duration-300 ease-in-out transform hover:-translate-y-1"
                        on:click=on_click
                    >
                        <span class="flex items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z" clip-rule="evenodd" />
                            </svg>
                            What does the API say?
                        </span>
                    </button>

                    <div class="w-full">
                        <div class="bg-gray-50 rounded-lg p-6 min-h-[100px] border border-gray-200">
                            <p class="text-center text-gray-700 font-medium">
                                {move || if value.get().is_empty() {
                                    "Response will appear here...".to_string()
                                } else {
                                    value.get()
                                }}
                            </p>
                        </div>
                    </div>

                    <div class="text-sm text-gray-500">
                        <p>Requests sent: {move || counter.get()}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
