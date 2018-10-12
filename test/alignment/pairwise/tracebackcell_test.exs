defmodule ExBio.Alignment.Pairwise.TracebackCellTest do
  use ExUnit.Case
  doctest ExBio
  alias ExBio.Alignment.Pairwise.TracebackCell

  test "new" do
    {:ok, tracebackcall} = TracebackCell.new()
    assert is_reference(tracebackcall)
  end

  test "set_i_bits and get_i_bits" do
    {:ok, tracebackcall} = TracebackCell.new()

    :ok = TracebackCell.set_i_bits(tracebackcall, 6)
    assert {:ok, 6} == TracebackCell.get_i_bits(tracebackcall)
  end
end
