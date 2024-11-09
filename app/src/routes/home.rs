use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {

    view! {
        <div class="min-h-screen bg-gray-100">
            // Hero Section
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto py-16 px-4 sm:px-6 lg:px-8">
                    <h1 class="text-4xl font-bold text-gray-900">"Hi, Im Khánh"</h1>
                    <p class="mt-4 text-xl text-gray-500">
                        "Exploring thoughts, ideas, and technology."
                    </p>
                </div>
            </header>
        </div>
    }
}
