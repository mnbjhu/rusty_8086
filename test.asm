bits 16

; ; Signed displacements
; mov ax, [bx + di - 37]
; mov [si - 300], cx
; mov dx, [bx - 32]
;
; ; Explicit sizes
; mov [bp + di], byte 7
; mov [di + 901], word 347

; Direct address
mov bp, [5]
mov bx, [3458]
