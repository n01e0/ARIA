BITS 32
    org 0x7C00

start:
    mov esi, msg
    call puts
    jmp 0

puts:
    mov al, [esi]
    inc esi
    cmp al, 0
    je puts_end
    mov ah, 0x0E
    mov ebx, 0x0B
    int 0x10
    jmp puts

puts_end:
    ret

msg:
    db "hello, world", 0x0D, 0x0A, 0
