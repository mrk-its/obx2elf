.global init_sprites

.text
init_sprites:
    ldx #<vbl
    ldy #>vbl
    jsr multi_init_engine
    rts

vbl:
    pla
    tay
    pla
    tax
    pla
    rti
