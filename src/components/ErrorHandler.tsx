import { createEffect, createSignal } from "solid-js";

function ErrorHandler(props: any) {
  const [error, setError] = createSignal(props.error);

  createEffect(() => {
    setError(props.error);
  }, error);

  return (
    <div role="alert" class="alert alert-warning break-all">
      <div class="first-letter:capitalize">{error()}</div>
    </div>
  );
}

export default ErrorHandler;
