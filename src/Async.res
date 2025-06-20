external requestAnimationFrame: (unit => unit) => unit = "requestAnimationFrame"

let requestAnimationFrame = () => Promise.make((res, _) => requestAnimationFrame(() => res()))
