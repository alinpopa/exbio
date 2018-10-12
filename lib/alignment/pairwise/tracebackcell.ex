defmodule ExBio.Alignment.Pairwise.TracebackCell do
  alias ExBio.Nif.RustBio

  def new(),
    do: RustBio.alignment_pairwise_tracebackcell_new()

  def set_i_bits(tracebackcell, value),
    do: RustBio.alignment_pairwise_tracebackcell_set_bits(tracebackcell, :i, value)

  def set_d_bits(tracebackcell, value),
    do: RustBio.alignment_pairwise_tracebackcell_set_bits(tracebackcell, :d, value)

  def set_s_bits(tracebackcell, value),
    do: RustBio.alignment_pairwise_tracebackcell_set_bits(tracebackcell, :s, value)

  def set_all(tracebackcell, value),
    do: RustBio.alignment_pairwise_tracebackcell_set_bits(tracebackcell, :all, value)

  def get_i_bits(tracebackcell) do
    case RustBio.alignment_pairwise_tracebackcell_get_bits(tracebackcell, :i) do
      {:ok, i: value} -> {:ok, value}
      _ -> {:error, :empty}
    end
  end

  def get_d_bits(tracebackcell) do
    case RustBio.alignment_pairwise_tracebackcell_get_bits(tracebackcell, :d) do
      {:ok, d: value} -> {:ok, value}
      _ -> {:error, :empty}
    end
  end

  def get_s_bits(tracebackcell) do
    case RustBio.alignment_pairwise_tracebackcell_get_bits(tracebackcell, :s) do
      {:ok, s: value} -> {:ok, value}
      _ -> {:error, :empty}
    end
  end
end
