import { createEffect, createSignal } from "solid-js";

export default function (props: any) {
  const [signal, setSignal] = createSignal(props.signal);
  createEffect(() => setSignal(props.signal))

  return (
    <div class="rating">
      <input type="radio" name="rating-1" class="mask mask-parallelogram" checked={signal() === 1} disabled />
      <input type="radio" name="rating-1" class="mask mask-parallelogram" checked={signal() === 2} disabled />
      <input type="radio" name="rating-1" class="mask mask-parallelogram" checked={signal() === 3} disabled />
      <input type="radio" name="rating-1" class="mask mask-parallelogram" checked={signal() === 4} disabled />
      <input type="radio" name="rating-1" class="mask mask-parallelogram" checked={signal() === 5} disabled />
    </div>
  )
}
