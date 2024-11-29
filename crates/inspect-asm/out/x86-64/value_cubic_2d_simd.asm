inspect_asm::value_cubic_2d_simd:
	movsd xmm1, qword ptr [rdi]
	cvttps2dq xmm2, xmm1
	xorps xmm0, xmm0
	cmpnleps xmm0, xmm1
	paddd xmm0, xmm2
	cvtdq2ps xmm2, xmm0
	subps xmm1, xmm2
	movq xmm4, qword ptr [rip + .LCPI27_8]
	pshufd xmm7, xmm0, 245
	pmuludq xmm0, xmm4
	pshufd xmm6, xmm0, 232
	pmuludq xmm7, xmmword ptr [rip + .LCPI27_1]
	pshufd xmm2, xmm7, 232
	punpckldq xmm6, xmm2
	movq xmm8, qword ptr [rip + .LCPI27_9]
	paddd xmm8, xmm6
	paddd xmm4, xmm6
	movq xmm9, qword ptr [rip + .LCPI27_10]
	paddd xmm9, xmm6
	pshufd xmm2, xmm8, 85
	pxor xmm2, xmm9
	movd eax, xmm2
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm2, xmm2
	cvtsi2ss xmm2, ecx
	movss xmm10, dword ptr [rip + .LCPI27_4]
	movdqa xmm3, xmm9
	pxor xmm3, xmm7
	movd eax, xmm3
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm3, xmm3
	cvtsi2ss xmm3, ecx
	mulss xmm2, xmm10
	pshufd xmm5, xmm4, 85
	pxor xmm5, xmm9
	movd eax, xmm5
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm5, xmm5
	cvtsi2ss xmm5, ecx
	mulss xmm3, xmm10
	pshufd xmm11, xmm9, 85
	pxor xmm11, xmm9
	movd eax, xmm11
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	cvtsi2ss xmm12, ecx
	mulss xmm5, xmm10
	mulss xmm12, xmm10
	pshufd xmm11, xmm8, 80
	punpckldq xmm9, xmm8
	pshufd xmm8, xmm9, 238
	movdqa xmm10, xmm11
	pxor xmm10, xmm8
	pshufd xmm13, xmm10, 85
	movdqa xmm9, xmmword ptr [rip + .LCPI27_5]
	pmuludq xmm13, xmm9
	pmuludq xmm10, xmm9
	pmuludq xmm10, xmm10
	pshufd xmm10, xmm10, 232
	pmuludq xmm13, xmm13
	pshufd xmm13, xmm13, 232
	punpckldq xmm10, xmm13
	movdqa xmm13, xmm10
	pslld xmm13, 19
	pxor xmm13, xmm10
	cvtdq2ps xmm14, xmm13
	movaps xmm10, xmmword ptr [rip + .LCPI27_6]
	mulps xmm14, xmm10
	shufps xmm0, xmm7, 0
	movdqa xmm7, xmm8
	pxor xmm7, xmm0
	pshufd xmm13, xmm7, 85
	pmuludq xmm13, xmm9
	pmuludq xmm7, xmm9
	pmuludq xmm7, xmm7
	pshufd xmm7, xmm7, 232
	pmuludq xmm13, xmm13
	pshufd xmm13, xmm13, 232
	punpckldq xmm7, xmm13
	movdqa xmm13, xmm7
	pslld xmm13, 19
	pxor xmm13, xmm7
	cvtdq2ps xmm7, xmm13
	mulps xmm7, xmm10
	pshufd xmm13, xmm4, 80
	pxor xmm8, xmm13
	pshufd xmm15, xmm8, 85
	pmuludq xmm15, xmm9
	pmuludq xmm8, xmm9
	pmuludq xmm8, xmm8
	pshufd xmm8, xmm8, 232
	pmuludq xmm15, xmm15
	pshufd xmm15, xmm15, 232
	punpckldq xmm8, xmm15
	movdqa xmm15, xmm8
	pslld xmm15, 19
	pxor xmm15, xmm8
	cvtdq2ps xmm8, xmm15
	mulps xmm8, xmm10
	movlhps xmm2, xmm14
	shufps xmm2, xmm14, 226
	movaps xmm15, xmm8
	shufps xmm15, xmm7, 1
	shufps xmm15, xmm7, 226
	subps xmm2, xmm15
	subss xmm12, xmm8
	subps xmm8, xmm14
	movss xmm14, xmm12
	movaps xmm12, xmm7
	shufps xmm12, xmm2, 1
	shufps xmm12, xmm2, 226
	subps xmm14, xmm12
	punpckldq xmm4, xmm6
	pshufd xmm4, xmm4, 238
	pxor xmm11, xmm4
	pshufd xmm6, xmm11, 85
	pmuludq xmm6, xmm9
	pmuludq xmm11, xmm9
	pmuludq xmm11, xmm11
	pshufd xmm11, xmm11, 232
	pmuludq xmm6, xmm6
	pshufd xmm6, xmm6, 232
	punpckldq xmm11, xmm6
	movdqa xmm6, xmm11
	pslld xmm6, 19
	pxor xmm6, xmm11
	pxor xmm0, xmm4
	pshufd xmm11, xmm0, 85
	pmuludq xmm11, xmm9
	pmuludq xmm0, xmm9
	pmuludq xmm0, xmm0
	pshufd xmm0, xmm0, 232
	pmuludq xmm11, xmm11
	pshufd xmm11, xmm11, 232
	punpckldq xmm0, xmm11
	movdqa xmm11, xmm0
	pslld xmm11, 19
	pxor xmm11, xmm0
	pxor xmm13, xmm4
	pshufd xmm0, xmm13, 85
	pmuludq xmm0, xmm9
	pmuludq xmm13, xmm9
	pmuludq xmm13, xmm13
	pshufd xmm4, xmm13, 232
	pmuludq xmm0, xmm0
	pshufd xmm0, xmm0, 232
	punpckldq xmm4, xmm0
	movdqa xmm0, xmm4
	pslld xmm0, 19
	pxor xmm0, xmm4
	cvtdq2ps xmm6, xmm6
	mulps xmm6, xmm10
	cvtdq2ps xmm4, xmm11
	mulps xmm4, xmm10
	cvtdq2ps xmm0, xmm0
	mulps xmm0, xmm10
	movlhps xmm3, xmm6
	shufps xmm3, xmm6, 226
	movaps xmm9, xmm0
	shufps xmm9, xmm4, 1
	shufps xmm9, xmm4, 226
	subps xmm3, xmm9
	movaps xmm9, xmm1
	mulps xmm9, xmm1
	subss xmm5, xmm0
	subps xmm0, xmm6
	movss xmm6, xmm5
	movaps xmm5, xmm4
	shufps xmm5, xmm3, 1
	shufps xmm5, xmm3, 226
	subps xmm6, xmm5
	movaps xmm5, xmm1
	mulps xmm5, xmm9
	subps xmm2, xmm14
	subps xmm3, xmm6
	unpcklps xmm5, xmm9
	movaps xmm9, xmm6
	subps xmm9, xmm3
	shufps xmm9, xmm6, 1
	shufps xmm9, xmm6, 226
	movaps xmm6, xmm14
	subps xmm6, xmm2
	shufps xmm6, xmm14, 197
	shufps xmm6, xmm14, 2
	mulps xmm9, xmm5
	mulps xmm6, xmm5
	movaps xmm10, xmm5
	shufps xmm10, xmm5, 225
	mulps xmm3, xmm10
	addps xmm3, xmm9
	movaps xmm5, xmm1
	shufps xmm1, xmm1, 0
	mulps xmm0, xmm1
	addps xmm0, xmm3
	addps xmm0, xmm4
	mulps xmm10, xmm2
	addps xmm10, xmm6
	shufps xmm5, xmm5, 85
	mulps xmm8, xmm1
	addps xmm8, xmm10
	addps xmm8, xmm7
	movaps xmm1, xmm8
	subps xmm1, xmm0
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 85
	subss xmm1, xmm2
	shufps xmm8, xmm8, 85
	movaps xmm3, xmm0
	subss xmm3, xmm8
	movaps xmm4, xmm5
	mulss xmm3, xmm5
	mulss xmm5, xmm5
	mulss xmm4, xmm5
	mulss xmm4, xmm1
	subss xmm2, xmm1
	mulss xmm2, xmm5
	addss xmm2, xmm4
	addss xmm3, xmm2
	shufps xmm0, xmm0, 85
	addss xmm0, xmm3
	mulss xmm0, dword ptr [rip + .LCPI27_7]
	ret