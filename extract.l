#!/usr/bin/env pil

(setq P (native "@" "malloc" 'N 32))

# initialize the struct
(struct P 'N
  32 (char "!")           # 2 bytes
  67 (char "D")           # 2 chars
  255 255 255 255         # int
  1 0 0 0 0 0 0 0         # long
  65 66 67 68 69 70 71 72 # string
  1 2 3 4 5 6 7 8 )       # char array

(let Res (native "target/debug/libpilrust.so" "extract" 'I P)
  (prinl "Result code: " Res)

  (if   (=0 Res)
        (let Extracted (struct P '((B . 2) (C . 2) I N S (B . 8)))
          (native "@" "free" NIL P)
          (prinl "Extracted struct: ")
          (pp 'Extracted)

          # tests
          (test (42 43) (car Extracted))
          (test '("A" "B") (; Extracted 2))
          (test 65535 (; Extracted 3))
          (test 9223372036854775807 (; Extracted 4))
          (test "pilrust" (; Extracted 5))
          (test "PicoLisp" (pack (mapcar char (last Extracted)))) )
        (bye 1) ]

(bye)
