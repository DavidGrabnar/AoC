defmodule Day1 do
  @moduledoc """
  Solution for `Day1`.
  """

  def run1(path) do
    {:ok, content} = File.read(path)

    content
    |> String.split("\n", trim: true)
    |> Enum.reduce({[], []}, fn line, {arr1, arr2} ->
      line
      |> String.replace("\r", "")
      |> String.split(" ", trim: true)
      |> Enum.map(&String.to_integer/1)
      |> (fn [a, b] -> {[a | arr1], [b | arr2]} end).()
    end)
    |> (fn {a, b} -> Enum.zip(Enum.sort(a), Enum.sort(b)) end).()
    |> Enum.reduce(0, fn {a, b}, acc ->
      abs(a - b) + acc
    end)
  end

  def run2(path) do
    {:ok, content} = File.read(path)

    {m1, m2} = content
     |> String.split("\n", trim: true)
     |> Enum.reduce({[], %{}}, fn line, {acc_m1, acc_m2} ->
      [a, b] =
        line
        |> String.replace("\r", "")
        |> String.split(" ", trim: true)
        |> Enum.map(&String.to_integer/1)

      new_m1 = [a | acc_m1]
      new_m2 = Map.update(acc_m2, b, 1, &(&1 + 1))

      {new_m1, new_m2}
    end)

    Enum.reduce(m1, 0, fn el, acc ->
      acc + el * Map.get(m2, el, 0)
    end)
  end
end
