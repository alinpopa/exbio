defmodule ExBio.PatternMatching.Bom do
  use Rustler, otp_app: :exbio, crate: :exbio

  # When your NIF is loaded, it will override this function.
  def bom(_text, _pattern), do: :erlang.nif_error(:nif_not_loaded)
end
