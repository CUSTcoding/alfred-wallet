function HomeComponent() {
  return (
    <main className="h-full rounded-2xl flex-1  gap-1 flex flex-col p-3 bg-zinc-900">
      <div className="w-full h-1/3 flex gap-5 p-2">
        <div className="transactionOnChain bg-amber-950 rounded-2xl h-full w-1/2">

        </div>
        <div className="transationLN bg-amber-950 rounded-2xl h-full w-1/2">

        </div>
      </div>

      <div className="w-full flex-1 flex items-center justify-around  gap-5 p-2">
        <div className="transactionOnChain bg-amber-950 rounded-2xl h-full w-1/2">

        </div>
        <div className="transationLN bg-amber-950 rounded-2xl h-full w-1/2">

        </div>
      </div>
    </main>
  );
}

export default HomeComponent;