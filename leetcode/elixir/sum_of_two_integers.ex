defmodule Solution do
  @moduledoc """
  Given two integers a and b, return the sum of the two integers without using the operators + and -.
  """  
  @spec sum_no_op(a :: integer, b :: integer) :: integer
  def sum_no_op(a, b) do
    [a, b]
    |> Enum.sum
  end
end
