function NoConnection() {
  return (
    <div class="flex flex-col portrait:h-3/4 landscape:h-2/4 justify-center">
      <div class="w-full text-center">
        Connecting...
      </div>
      <div class="w-full text-center pt-4">
        <span class="loading loading-dots loading-md"></span>
      </div>
    </div>
  );
}

export default NoConnection;
