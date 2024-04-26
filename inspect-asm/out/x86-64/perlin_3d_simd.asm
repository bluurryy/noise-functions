inspect_asm::perlin_3d_simd:
	movaps xmm2, xmmword ptr [rdi]
	cvttps2dq xmm0, xmm2
	xorps xmm4, xmm4
	cmpnleps xmm4, xmm2
	paddd xmm4, xmm0
	cvtdq2ps xmm0, xmm4
	subps xmm2, xmm0
	movaps xmm0, xmmword ptr [rip + .LCPI24_0]
	addps xmm0, xmm2
	movaps xmm3, xmm2
	mulps xmm3, xmm2
	movaps xmm1, xmmword ptr [rip + .LCPI24_1]
	mulps xmm1, xmm2
	addps xmm1, xmmword ptr [rip + .LCPI24_2]
	mulps xmm3, xmm2
	mulps xmm1, xmm2
	addps xmm1, xmmword ptr [rip + .LCPI24_3]
	mulps xmm1, xmm3
	pshufd xmm5, xmm4, 245
	pmuludq xmm4, xmmword ptr [rip + .LCPI24_4]
	pshufd xmm3, xmm4, 232
	pmuludq xmm5, xmmword ptr [rip + .LCPI24_5]
	pshufd xmm5, xmm5, 232
	punpckldq xmm3, xmm5
	movdqa xmm5, xmmword ptr [rip + .LCPI24_6]
	paddd xmm5, xmm3
	movdqa xmm6, xmm3
	shufps xmm6, xmm5, 17
	shufps xmm6, xmm5, 104
	pshufd xmm7, xmm5, 0
	shufps xmm5, xmm3, 18
	shufps xmm3, xmm5, 42
	xorps xmm3, xmm6
	pshufd xmm4, xmm4, 0
	pxor xmm4, xmm3
	movdqa xmm6, xmmword ptr [rip + .LCPI24_7]
	pshufd xmm5, xmm4, 245
	pmuludq xmm4, xmm6
	pshufd xmm4, xmm4, 232
	pmuludq xmm5, xmm6
	pshufd xmm5, xmm5, 232
	punpckldq xmm4, xmm5
	movdqa xmm5, xmm4
	psrld xmm5, 15
	pxor xmm5, xmm4
	movdqa xmm8, xmmword ptr [rip + .LCPI24_8]
	pand xmm5, xmm8
	movd ecx, xmm5
	lea rax, [rip + .L__unnamed__0]
	movaps xmm4, xmmword ptr [rax + 4*rcx]
	pshufd xmm9, xmm5, 85
	movd ecx, xmm9
	movaps xmm9, xmmword ptr [rax + 4*rcx]
	pshufd xmm10, xmm5, 238
	movd ecx, xmm10
	movaps xmm10, xmmword ptr [rax + 4*rcx]
	pshufd xmm5, xmm5, 255
	movd ecx, xmm5
	movaps xmm5, xmmword ptr [rax + 4*rcx]
	movaps xmm11, xmm9
	shufps xmm11, xmm4, 16
	movaps xmm12, xmm5
	shufps xmm12, xmm10, 212
	shufps xmm11, xmm12, 34
	movaps xmm12, xmm2
	shufps xmm12, xmm2, 17
	mulps xmm12, xmm11
	movaps xmm11, xmm9
	shufps xmm11, xmm4, 1
	movaps xmm13, xmm5
	shufps xmm13, xmm10, 197
	shufps xmm11, xmm13, 34
	movaps xmm13, xmm2
	shufps xmm13, xmm0, 80
	shufps xmm13, xmm13, 216
	mulps xmm13, xmm11
	addps xmm13, xmm12
	unpckhps xmm4, xmm9
	unpckhpd xmm5, xmm10
	shufps xmm4, xmm5, 36
	movaps xmm5, xmm2
	shufps xmm5, xmm0, 170
	mulps xmm4, xmm5
	addps xmm4, xmm13
	pxor xmm7, xmm3
	pshufd xmm3, xmm7, 245
	pmuludq xmm7, xmm6
	pshufd xmm7, xmm7, 232
	pmuludq xmm3, xmm6
	pshufd xmm3, xmm3, 232
	punpckldq xmm7, xmm3
	movdqa xmm10, xmm7
	psrld xmm10, 15
	pxor xmm10, xmm7
	pand xmm10, xmm8
	movd ecx, xmm10
	movaps xmm3, xmmword ptr [rax + 4*rcx]
	pshufd xmm6, xmm10, 85
	movd ecx, xmm6
	movaps xmm6, xmmword ptr [rax + 4*rcx]
	pshufd xmm7, xmm10, 238
	movd ecx, xmm7
	movaps xmm9, xmmword ptr [rax + 4*rcx]
	pshufd xmm7, xmm10, 255
	movd ecx, xmm7
	movaps xmm7, xmmword ptr [rax + 4*rcx]
	movaps xmm8, xmm2
	shufps xmm8, xmm0, 17
	movaps xmm10, xmm7
	movaps xmm11, xmm7
	shufps xmm11, xmm9, 85
	unpckhpd xmm7, xmm9
	movlhps xmm9, xmm0
	shufps xmm8, xmm9, 136
	movaps xmm9, xmm6
	shufps xmm9, xmm3, 17
	movlhps xmm10, xmm0
	shufps xmm9, xmm10, 34
	mulps xmm9, xmm8
	movaps xmm8, xmm3
	unpcklps xmm8, xmm0
	shufps xmm2, xmm0, 85
	shufps xmm8, xmm2, 132
	unpcklps xmm0, xmm6
	shufps xmm0, xmm11, 36
	mulps xmm0, xmm8
	addps xmm0, xmm9
	unpckhps xmm3, xmm6
	shufps xmm3, xmm7, 36
	mulps xmm3, xmm5
	addps xmm3, xmm0
	movaps xmm0, xmm1
	shufps xmm0, xmm1, 0
	subps xmm3, xmm4
	mulps xmm3, xmm0
	addps xmm3, xmm4
	movaps xmm2, xmm3
	shufps xmm2, xmm3, 231
	shufps xmm3, xmm3, 226
	subps xmm2, xmm3
	movaps xmm0, xmm1
	shufps xmm0, xmm1, 85
	mulps xmm0, xmm2
	addps xmm0, xmm3
	movhlps xmm1, xmm1
	movaps xmm2, xmm0
	shufps xmm2, xmm0, 85
	subss xmm0, xmm2
	mulss xmm0, xmm1
	addss xmm0, xmm2
	mulss xmm0, dword ptr [rip + .LCPI24_9]
	ret