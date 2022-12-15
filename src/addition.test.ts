// requireActual ensures you get the real file
// instead of an automock
// we use import type and <typeof ...> to still get types
import type * as Add from "./addition"
const { addition } = jest.requireActual<typeof Add>("../addition")

describe("addition function", () => {
  test("two plus two is four", () => {
    expect(addition(2, 2)).toBe(4)
  })
})

// required with this small example
// to make the isolatedModules config happy
export {}