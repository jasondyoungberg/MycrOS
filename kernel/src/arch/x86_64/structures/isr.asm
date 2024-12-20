.macro push_all
    push r15
    push r14
    push r13
    push r12
    push r11
    push r10
    push r9
    push r8
    push rsp
    push rbp
    push rdi
    push rsi
    push rdx
    push rcx
    push rbx
    push rax
.endmacro

.macro pop_all
    pop rax
    pop rbx
    pop rcx
    pop rdx
    pop rsi
    pop rdi
    pop rbp
    pop rsp
    pop r8
    pop r9
    pop r10
    pop r11
    pop r12
    pop r13
    pop r14
    pop r15
.endmacro

.macro swapgs_if_necessary
    cmp qword ptr [rsp + 24], 8
    je 2f
    swapgs
    2:
.endmacro

.macro def_isr_stub n
    isr_stub_\n:
    push 0
    push \n
    jmp isr_main
.endmacro

.macro def_isr_stub_err n
    isr_stub_\n:
    push \n
    jmp isr_main
.endmacro

.section .text

.extern isr_inner
isr_main:
    swapgs_if_necessary
    push_all
    mov rdi, rsp
    sub rsp, 8 // alignment
    cld
    call isr_inner
    add rsp, 8
    pop_all
    swapgs_if_necessary
    add rsp, 16
    iretq

def_isr_stub 0
def_isr_stub 1
def_isr_stub 2
def_isr_stub 3
def_isr_stub 4
def_isr_stub 5
def_isr_stub 6
def_isr_stub 7
def_isr_stub_err 8
def_isr_stub 9
def_isr_stub_err 10
def_isr_stub_err 11
def_isr_stub_err 12
def_isr_stub_err 13
def_isr_stub_err 14
def_isr_stub 15
def_isr_stub 16
def_isr_stub_err 17
def_isr_stub 18
def_isr_stub 19
def_isr_stub 20
def_isr_stub_err 21
def_isr_stub 22
def_isr_stub 23
def_isr_stub 24
def_isr_stub 25
def_isr_stub 26
def_isr_stub 27
def_isr_stub 28
def_isr_stub_err 29
def_isr_stub 30
def_isr_stub 31
def_isr_stub 32
def_isr_stub 33
def_isr_stub 34
def_isr_stub 35
def_isr_stub 36
def_isr_stub 37
def_isr_stub 38
def_isr_stub 39
def_isr_stub 40
def_isr_stub 41
def_isr_stub 42
def_isr_stub 43
def_isr_stub 44
def_isr_stub 45
def_isr_stub 46
def_isr_stub 47
def_isr_stub 48
def_isr_stub 49
def_isr_stub 50
def_isr_stub 51
def_isr_stub 52
def_isr_stub 53
def_isr_stub 54
def_isr_stub 55
def_isr_stub 56
def_isr_stub 57
def_isr_stub 58
def_isr_stub 59
def_isr_stub 60
def_isr_stub 61
def_isr_stub 62
def_isr_stub 63
def_isr_stub 64
def_isr_stub 65
def_isr_stub 66
def_isr_stub 67
def_isr_stub 68
def_isr_stub 69
def_isr_stub 70
def_isr_stub 71
def_isr_stub 72
def_isr_stub 73
def_isr_stub 74
def_isr_stub 75
def_isr_stub 76
def_isr_stub 77
def_isr_stub 78
def_isr_stub 79
def_isr_stub 80
def_isr_stub 81
def_isr_stub 82
def_isr_stub 83
def_isr_stub 84
def_isr_stub 85
def_isr_stub 86
def_isr_stub 87
def_isr_stub 88
def_isr_stub 89
def_isr_stub 90
def_isr_stub 91
def_isr_stub 92
def_isr_stub 93
def_isr_stub 94
def_isr_stub 95
def_isr_stub 96
def_isr_stub 97
def_isr_stub 98
def_isr_stub 99
def_isr_stub 100
def_isr_stub 101
def_isr_stub 102
def_isr_stub 103
def_isr_stub 104
def_isr_stub 105
def_isr_stub 106
def_isr_stub 107
def_isr_stub 108
def_isr_stub 109
def_isr_stub 110
def_isr_stub 111
def_isr_stub 112
def_isr_stub 113
def_isr_stub 114
def_isr_stub 115
def_isr_stub 116
def_isr_stub 117
def_isr_stub 118
def_isr_stub 119
def_isr_stub 120
def_isr_stub 121
def_isr_stub 122
def_isr_stub 123
def_isr_stub 124
def_isr_stub 125
def_isr_stub 126
def_isr_stub 127
def_isr_stub 128
def_isr_stub 129
def_isr_stub 130
def_isr_stub 131
def_isr_stub 132
def_isr_stub 133
def_isr_stub 134
def_isr_stub 135
def_isr_stub 136
def_isr_stub 137
def_isr_stub 138
def_isr_stub 139
def_isr_stub 140
def_isr_stub 141
def_isr_stub 142
def_isr_stub 143
def_isr_stub 144
def_isr_stub 145
def_isr_stub 146
def_isr_stub 147
def_isr_stub 148
def_isr_stub 149
def_isr_stub 150
def_isr_stub 151
def_isr_stub 152
def_isr_stub 153
def_isr_stub 154
def_isr_stub 155
def_isr_stub 156
def_isr_stub 157
def_isr_stub 158
def_isr_stub 159
def_isr_stub 160
def_isr_stub 161
def_isr_stub 162
def_isr_stub 163
def_isr_stub 164
def_isr_stub 165
def_isr_stub 166
def_isr_stub 167
def_isr_stub 168
def_isr_stub 169
def_isr_stub 170
def_isr_stub 171
def_isr_stub 172
def_isr_stub 173
def_isr_stub 174
def_isr_stub 175
def_isr_stub 176
def_isr_stub 177
def_isr_stub 178
def_isr_stub 179
def_isr_stub 180
def_isr_stub 181
def_isr_stub 182
def_isr_stub 183
def_isr_stub 184
def_isr_stub 185
def_isr_stub 186
def_isr_stub 187
def_isr_stub 188
def_isr_stub 189
def_isr_stub 190
def_isr_stub 191
def_isr_stub 192
def_isr_stub 193
def_isr_stub 194
def_isr_stub 195
def_isr_stub 196
def_isr_stub 197
def_isr_stub 198
def_isr_stub 199
def_isr_stub 200
def_isr_stub 201
def_isr_stub 202
def_isr_stub 203
def_isr_stub 204
def_isr_stub 205
def_isr_stub 206
def_isr_stub 207
def_isr_stub 208
def_isr_stub 209
def_isr_stub 210
def_isr_stub 211
def_isr_stub 212
def_isr_stub 213
def_isr_stub 214
def_isr_stub 215
def_isr_stub 216
def_isr_stub 217
def_isr_stub 218
def_isr_stub 219
def_isr_stub 220
def_isr_stub 221
def_isr_stub 222
def_isr_stub 223
def_isr_stub 224
def_isr_stub 225
def_isr_stub 226
def_isr_stub 227
def_isr_stub 228
def_isr_stub 229
def_isr_stub 230
def_isr_stub 231
def_isr_stub 232
def_isr_stub 233
def_isr_stub 234
def_isr_stub 235
def_isr_stub 236
def_isr_stub 237
def_isr_stub 238
def_isr_stub 239
def_isr_stub 240
def_isr_stub 241
def_isr_stub 242
def_isr_stub 243
def_isr_stub 244
def_isr_stub 245
def_isr_stub 246
def_isr_stub 247
def_isr_stub 248
def_isr_stub 249
def_isr_stub 250
def_isr_stub 251
def_isr_stub 252
def_isr_stub 253
def_isr_stub 254
def_isr_stub 255

.section .rodata

.align 8
.global isr_table
isr_table:
    .quad isr_stub_0
    .quad isr_stub_1
    .quad isr_stub_2
    .quad isr_stub_3
    .quad isr_stub_4
    .quad isr_stub_5
    .quad isr_stub_6
    .quad isr_stub_7
    .quad isr_stub_8
    .quad isr_stub_9
    .quad isr_stub_10
    .quad isr_stub_11
    .quad isr_stub_12
    .quad isr_stub_13
    .quad isr_stub_14
    .quad isr_stub_15
    .quad isr_stub_16
    .quad isr_stub_17
    .quad isr_stub_18
    .quad isr_stub_19
    .quad isr_stub_20
    .quad isr_stub_21
    .quad isr_stub_22
    .quad isr_stub_23
    .quad isr_stub_24
    .quad isr_stub_25
    .quad isr_stub_26
    .quad isr_stub_27
    .quad isr_stub_28
    .quad isr_stub_29
    .quad isr_stub_30
    .quad isr_stub_31
    .quad isr_stub_32
    .quad isr_stub_33
    .quad isr_stub_34
    .quad isr_stub_35
    .quad isr_stub_36
    .quad isr_stub_37
    .quad isr_stub_38
    .quad isr_stub_39
    .quad isr_stub_40
    .quad isr_stub_41
    .quad isr_stub_42
    .quad isr_stub_43
    .quad isr_stub_44
    .quad isr_stub_45
    .quad isr_stub_46
    .quad isr_stub_47
    .quad isr_stub_48
    .quad isr_stub_49
    .quad isr_stub_50
    .quad isr_stub_51
    .quad isr_stub_52
    .quad isr_stub_53
    .quad isr_stub_54
    .quad isr_stub_55
    .quad isr_stub_56
    .quad isr_stub_57
    .quad isr_stub_58
    .quad isr_stub_59
    .quad isr_stub_60
    .quad isr_stub_61
    .quad isr_stub_62
    .quad isr_stub_63
    .quad isr_stub_64
    .quad isr_stub_65
    .quad isr_stub_66
    .quad isr_stub_67
    .quad isr_stub_68
    .quad isr_stub_69
    .quad isr_stub_70
    .quad isr_stub_71
    .quad isr_stub_72
    .quad isr_stub_73
    .quad isr_stub_74
    .quad isr_stub_75
    .quad isr_stub_76
    .quad isr_stub_77
    .quad isr_stub_78
    .quad isr_stub_79
    .quad isr_stub_80
    .quad isr_stub_81
    .quad isr_stub_82
    .quad isr_stub_83
    .quad isr_stub_84
    .quad isr_stub_85
    .quad isr_stub_86
    .quad isr_stub_87
    .quad isr_stub_88
    .quad isr_stub_89
    .quad isr_stub_90
    .quad isr_stub_91
    .quad isr_stub_92
    .quad isr_stub_93
    .quad isr_stub_94
    .quad isr_stub_95
    .quad isr_stub_96
    .quad isr_stub_97
    .quad isr_stub_98
    .quad isr_stub_99
    .quad isr_stub_100
    .quad isr_stub_101
    .quad isr_stub_102
    .quad isr_stub_103
    .quad isr_stub_104
    .quad isr_stub_105
    .quad isr_stub_106
    .quad isr_stub_107
    .quad isr_stub_108
    .quad isr_stub_109
    .quad isr_stub_110
    .quad isr_stub_111
    .quad isr_stub_112
    .quad isr_stub_113
    .quad isr_stub_114
    .quad isr_stub_115
    .quad isr_stub_116
    .quad isr_stub_117
    .quad isr_stub_118
    .quad isr_stub_119
    .quad isr_stub_120
    .quad isr_stub_121
    .quad isr_stub_122
    .quad isr_stub_123
    .quad isr_stub_124
    .quad isr_stub_125
    .quad isr_stub_126
    .quad isr_stub_127
    .quad isr_stub_128
    .quad isr_stub_129
    .quad isr_stub_130
    .quad isr_stub_131
    .quad isr_stub_132
    .quad isr_stub_133
    .quad isr_stub_134
    .quad isr_stub_135
    .quad isr_stub_136
    .quad isr_stub_137
    .quad isr_stub_138
    .quad isr_stub_139
    .quad isr_stub_140
    .quad isr_stub_141
    .quad isr_stub_142
    .quad isr_stub_143
    .quad isr_stub_144
    .quad isr_stub_145
    .quad isr_stub_146
    .quad isr_stub_147
    .quad isr_stub_148
    .quad isr_stub_149
    .quad isr_stub_150
    .quad isr_stub_151
    .quad isr_stub_152
    .quad isr_stub_153
    .quad isr_stub_154
    .quad isr_stub_155
    .quad isr_stub_156
    .quad isr_stub_157
    .quad isr_stub_158
    .quad isr_stub_159
    .quad isr_stub_160
    .quad isr_stub_161
    .quad isr_stub_162
    .quad isr_stub_163
    .quad isr_stub_164
    .quad isr_stub_165
    .quad isr_stub_166
    .quad isr_stub_167
    .quad isr_stub_168
    .quad isr_stub_169
    .quad isr_stub_170
    .quad isr_stub_171
    .quad isr_stub_172
    .quad isr_stub_173
    .quad isr_stub_174
    .quad isr_stub_175
    .quad isr_stub_176
    .quad isr_stub_177
    .quad isr_stub_178
    .quad isr_stub_179
    .quad isr_stub_180
    .quad isr_stub_181
    .quad isr_stub_182
    .quad isr_stub_183
    .quad isr_stub_184
    .quad isr_stub_185
    .quad isr_stub_186
    .quad isr_stub_187
    .quad isr_stub_188
    .quad isr_stub_189
    .quad isr_stub_190
    .quad isr_stub_191
    .quad isr_stub_192
    .quad isr_stub_193
    .quad isr_stub_194
    .quad isr_stub_195
    .quad isr_stub_196
    .quad isr_stub_197
    .quad isr_stub_198
    .quad isr_stub_199
    .quad isr_stub_200
    .quad isr_stub_201
    .quad isr_stub_202
    .quad isr_stub_203
    .quad isr_stub_204
    .quad isr_stub_205
    .quad isr_stub_206
    .quad isr_stub_207
    .quad isr_stub_208
    .quad isr_stub_209
    .quad isr_stub_210
    .quad isr_stub_211
    .quad isr_stub_212
    .quad isr_stub_213
    .quad isr_stub_214
    .quad isr_stub_215
    .quad isr_stub_216
    .quad isr_stub_217
    .quad isr_stub_218
    .quad isr_stub_219
    .quad isr_stub_220
    .quad isr_stub_221
    .quad isr_stub_222
    .quad isr_stub_223
    .quad isr_stub_224
    .quad isr_stub_225
    .quad isr_stub_226
    .quad isr_stub_227
    .quad isr_stub_228
    .quad isr_stub_229
    .quad isr_stub_230
    .quad isr_stub_231
    .quad isr_stub_232
    .quad isr_stub_233
    .quad isr_stub_234
    .quad isr_stub_235
    .quad isr_stub_236
    .quad isr_stub_237
    .quad isr_stub_238
    .quad isr_stub_239
    .quad isr_stub_240
    .quad isr_stub_241
    .quad isr_stub_242
    .quad isr_stub_243
    .quad isr_stub_244
    .quad isr_stub_245
    .quad isr_stub_246
    .quad isr_stub_247
    .quad isr_stub_248
    .quad isr_stub_249
    .quad isr_stub_250
    .quad isr_stub_251
    .quad isr_stub_252
    .quad isr_stub_253
    .quad isr_stub_254
    .quad isr_stub_255
