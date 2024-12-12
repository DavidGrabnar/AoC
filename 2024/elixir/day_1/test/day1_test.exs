defmodule Day1Test do
  use ExUnit.Case
  doctest Day1

  test "step 1 - 1" do
    assert Day1.run1("../../inputs/day-1-1.txt") == 11
  end

  test "step 1 - 2" do
    assert Day1.run1("../../inputs/day-1-2.txt") == 2000468
  end

  test "step 2 - 1" do
    assert Day1.run2("../../inputs/day-1-1.txt") == 31
  end

  test "step 2 - 2" do
    assert Day1.run2("../../inputs/day-1-2.txt") == 18567089
  end

end
