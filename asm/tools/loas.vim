" Vim syntax file
" Language: loas - lovm assembly
" Maintainer: lausek
" Latest Revision: 24.04.2019

if exists("b:current_syntax")
    finish
endif

setlocal tabstop=4
setlocal softtabstop=4
setlocal shiftwidth=4
setlocal completefunc=syntaxcomplete#Complete

highlight loasComment   ctermfg=8       guifg=#0A0A0A
highlight loasConst     ctermfg=13      guifg=#00ffff
highlight loasKeyword   ctermfg=red     guifg=#00ffff
highlight loasMacro     ctermfg=6       guifg=#00ffff
highlight loasString    ctermfg=yellow  guifg=#00ffff

syntax match loasComment /;.*$/

syntax match loasConst "\v<\d+>"
syntax match loasConst "\v<(true|false)>"

syntax match loasMacro "\v\.<\S+>"

syntax region loasString start=/"/ skip =/\\"/ end=/"/

syntax keyword loasKeyword
    \ inc
    \ dec
    \ add
    \ sub
    \ mul
    \ div
    \ rem
    \ pow
    \ neg
    \ and
    \ or
    \ xor
    \ shl
    \ shr
    \ cmp
    \ jmp
    \ jeq
    \ jne
    \ jge
    \ jgt
    \ jle
    \ jlt
    \ cast
    \ call
    \ int
    \ ret
    \ push
    \ pop
    \ pusha
    \ popa
    \ dv
    \ mov
