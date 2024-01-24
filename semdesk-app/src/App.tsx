import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { ResultList } from "./ResultList";

function App() {
  const [name, setName] = useState("");
  const [data, setData] = useState<any[]>([]);

  async function greet() {
    const r = await fetch(`http://localhost:8080/search?query=${name}`, {
      headers: { "Content-Type": "application/json" },
    });
    setData((await r.json())["results"]);
  }

  return (
    <>
      <div className="sticky top-0 w-full bg-slate-300 p-4 z-0">
        <div className="flex items-center">
          <h1 className="flex-1 text-3xl font-bold">semdesk</h1>

          <form
            className="row"
            onSubmit={(e) => {
              e.preventDefault();
              greet();
            }}
          >
            <div className="flex items-center gap-2">
              <div className="bg-white flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600 sm:max-w-md">
                <input
                  type="text"
                  name="username"
                  id="username"
                  placeholder="Something cool..."
                  className="block flex-1 border-0 bg-transparent py-1.5 pl-1 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                  onChange={(e) => setName(e.currentTarget.value)}
                />
              </div>
              <button
                className="rounded bg-indigo-600 px-2 py-1 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
                type="submit"
              >
                Search
              </button>
            </div>
          </form>
        </div>
      </div>

      <div className="container mx-auto px-2 py-4">
        {data && <ResultList results={data} />}
      </div>
    </>
  );
}

export default App;
