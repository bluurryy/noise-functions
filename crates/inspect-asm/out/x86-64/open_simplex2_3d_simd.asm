inspect_asm::open_simplex2_3d_simd:
	movaps xmm0, xmmword ptr [rdi]
	movaps xmm1, xmm0
	shufps xmm1, xmm0, 85
	addss xmm1, xmm0
	movaps xmm5, xmm0
	unpckhpd xmm5, xmm0
	addss xmm5, xmm1
	mulss xmm5, dword ptr [rip + .LCPI16_0]
	shufps xmm5, xmm5, 0
	subps xmm5, xmm0
	xorps xmm0, xmm0
	cmpleps xmm0, xmm5
	movaps xmm1, xmm0
	andnps xmm1, xmmword ptr [rip + .LCPI16_1]
	andps xmm0, xmmword ptr [rip + .LCPI16_2]
	orps xmm0, xmm1
	addps xmm0, xmm5
	cvttps2dq xmm0, xmm0
	cvtdq2ps xmm1, xmm0
	subps xmm5, xmm1
	movaps xmm1, xmmword ptr [rip + .LCPI16_3]
	subps xmm1, xmm5
	movaps xmm3, xmm1
	shufps xmm3, xmm1, 255
	cvttss2si edx, xmm3
	movss xmm2, dword ptr [rip + .LCPI16_4]
	ucomiss xmm3, xmm2
	mov ecx, 2147483647
	cmova edx, ecx
	xor eax, eax
	ucomiss xmm3, xmm3
	cmovp edx, eax
	movaps xmm3, xmm1
	unpckhpd xmm3, xmm1
	cvttss2si esi, xmm3
	ucomiss xmm3, xmm2
	cmova esi, ecx
	ucomiss xmm3, xmm3
	cmovp esi, eax
	cvttss2si edi, xmm1
	ucomiss xmm1, xmm2
	cmova edi, ecx
	ucomiss xmm1, xmm1
	movd xmm3, edx
	cmovp edi, eax
	shufps xmm1, xmm1, 85
	cvttss2si edx, xmm1
	ucomiss xmm1, xmm2
	movd xmm4, esi
	cmova edx, ecx
	punpckldq xmm4, xmm3
	movd xmm2, edi
	ucomiss xmm1, xmm1
	cmovp edx, eax
	movd xmm1, edx
	punpckldq xmm2, xmm1
	punpcklqdq xmm2, xmm4
	movdqa xmm4, xmmword ptr [rip + .LCPI16_5]
	por xmm4, xmm2
	cvtdq2ps xmm1, xmm4
	movaps xmm7, xmmword ptr [rip + .LCPI16_6]
	xorps xmm7, xmm5
	mulps xmm7, xmm1
	pshufd xmm3, xmm0, 245
	pmuludq xmm0, xmmword ptr [rip + .LCPI16_7]
	pmuludq xmm3, xmmword ptr [rip + .LCPI16_8]
	pshufd xmm6, xmm0, 232
	pshufd xmm0, xmm3, 232
	punpckldq xmm6, xmm0
	movaps xmm0, xmm5
	mulss xmm0, xmm5
	movss xmm3, dword ptr [rip + .LCPI16_9]
	subss xmm3, xmm0
	movaps xmm9, xmm5
	shufps xmm9, xmm5, 85
	movaps xmm0, xmm9
	mulss xmm0, xmm9
	movaps xmm10, xmm5
	unpckhpd xmm10, xmm5
	movaps xmm8, xmm10
	mulss xmm8, xmm10
	addss xmm8, xmm0
	subss xmm3, xmm8
	xorps xmm0, xmm0
	ucomiss xmm3, xmm0
	jbe .LBB_2
	movaps xmm0, xmm3
	mulss xmm0, xmm3
	mulss xmm0, xmm0
	pshufd xmm8, xmm6, 85
	pshufd xmm11, xmm6, 238
	pxor xmm11, xmm8
	pxor xmm11, xmm6
	movd eax, xmm11
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	movaps xmm8, xmmword ptr [rax + 4*rcx]
	mulps xmm8, xmm5
	movaps xmm11, xmm8
	shufps xmm11, xmm8, 85
	addss xmm11, xmm8
	movhlps xmm8, xmm8
	addss xmm8, xmm11
	mulss xmm8, xmm0
	xorps xmm0, xmm0
	addss xmm0, xmm8
.LBB_2:
	movss xmm8, dword ptr [rip + .LCPI16_10]
	addss xmm8, xmm3
	movaps xmm11, xmm7
	shufps xmm11, xmm7, 85
	ucomiss xmm7, xmm11
	movaps xmm12, xmm7
	unpckhpd xmm12, xmm7
	jb .LBB_4
	ucomiss xmm7, xmm12
	jb .LBB_4
	movd eax, xmm4
	movaps xmm9, xmm5
	addss xmm9, xmm1
	movss xmm5, xmm9
	movaps xmm11, xmm1
	addss xmm11, xmm1
	mulss xmm11, xmm9
	movd ecx, xmm6
	imul eax, eax, -501125321
	add eax, ecx
	movd xmm9, eax
	movdqa xmm10, xmm6
	movss xmm10, xmm9
	subss xmm8, xmm11
	xorps xmm9, xmm9
	ucomiss xmm8, xmm9
	ja .LBB_9
	jmp .LBB_10
.LBB_4:
	ucomiss xmm11, xmm7
	jbe .LBB_6
	ucomiss xmm11, xmm12
	jb .LBB_6
	pshufd xmm10, xmm4, 85
	movd eax, xmm10
	cvtdq2ps xmm11, xmm10
	addss xmm9, xmm11
	movaps xmm12, xmm9
	movlhps xmm12, xmm5
	shufps xmm12, xmm5, 226
	addss xmm11, xmm11
	mulss xmm11, xmm9
	pshufd xmm5, xmm6, 85
	movd ecx, xmm5
	imul eax, eax, -1136930381
	add eax, ecx
	movd xmm10, eax
	punpcklqdq xmm10, xmm6
	shufps xmm10, xmm6, 226
	movaps xmm5, xmm12
	subss xmm8, xmm11
	xorps xmm9, xmm9
	ucomiss xmm8, xmm9
	ja .LBB_9
	jmp .LBB_10
.LBB_6:
	pshufd xmm9, xmm4, 238
	movd eax, xmm9
	cvtdq2ps xmm11, xmm9
	addss xmm10, xmm11
	movaps xmm9, xmm10
	shufps xmm9, xmm5, 48
	shufps xmm5, xmm9, 132
	addss xmm11, xmm11
	mulss xmm11, xmm10
	pshufd xmm9, xmm6, 238
	movd ecx, xmm9
	imul eax, eax, -1720413743
	add eax, ecx
	movd xmm9, eax
	shufps xmm9, xmm6, 48
	movaps xmm10, xmm6
	shufps xmm10, xmm9, 132
	subss xmm8, xmm11
	xorps xmm9, xmm9
	ucomiss xmm8, xmm9
	jbe .LBB_10
.LBB_9:
	mulss xmm8, xmm8
	pshufd xmm11, xmm10, 85
	pshufd xmm12, xmm10, 238
	pxor xmm12, xmm11
	pxor xmm12, xmm10
	movd eax, xmm12
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	mulps xmm5, xmmword ptr [rax + 4*rcx]
	mulss xmm8, xmm8
	movaps xmm10, xmm5
	shufps xmm10, xmm5, 85
	addss xmm10, xmm5
	movhlps xmm5, xmm5
	addss xmm5, xmm10
	mulss xmm5, xmm8
	addss xmm0, xmm5
.LBB_10:
	movaps xmm5, xmmword ptr [rip + .LCPI16_2]
	subps xmm5, xmm7
	mulps xmm1, xmm5
	movss xmm10, dword ptr [rip + .LCPI16_11]
	subss xmm10, xmm5
	movaps xmm7, xmm5
	shufps xmm7, xmm5, 85
	movaps xmm8, xmm5
	unpckhpd xmm8, xmm5
	movaps xmm11, xmm7
	addss xmm11, xmm8
	subss xmm10, xmm11
	addss xmm3, xmm10
	psrld xmm2, 1
	pand xmm2, xmmword ptr [rip + .LCPI16_7]
	paddd xmm2, xmm6
	pxor xmm6, xmm6
	ucomiss xmm3, xmm9
	jbe .LBB_12
	movaps xmm9, xmm3
	mulss xmm9, xmm3
	mulss xmm9, xmm9
	pshufd xmm10, xmm2, 85
	pshufd xmm11, xmm2, 238
	pxor xmm11, xmm10
	pxor xmm11, xmm2
	movd eax, xmm11
	not eax
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	movaps xmm10, xmmword ptr [rax + 4*rcx]
	mulps xmm10, xmm1
	movaps xmm11, xmm10
	shufps xmm11, xmm10, 85
	addss xmm11, xmm10
	movhlps xmm10, xmm10
	addss xmm10, xmm11
	mulss xmm10, xmm9
	addss xmm0, xmm10
.LBB_12:
	psubd xmm6, xmm4
	addss xmm3, dword ptr [rip + .LCPI16_10]
	ucomiss xmm5, xmm7
	jb .LBB_15
	ucomiss xmm5, xmm8
	jb .LBB_15
	movd eax, xmm6
	cvtdq2ps xmm4, xmm6
	movaps xmm5, xmm1
	addss xmm5, xmm4
	addss xmm1, xmm4
	addss xmm4, xmm4
	mulss xmm4, xmm5
	movd ecx, xmm2
	imul eax, eax, -501125321
	add eax, ecx
	movd xmm5, eax
	movss xmm2, xmm5
	subss xmm3, xmm4
	xorps xmm4, xmm4
	ucomiss xmm3, xmm4
	ja .LBB_20
	jmp .LBB_21
.LBB_15:
	ucomiss xmm7, xmm5
	jbe .LBB_18
	ucomiss xmm7, xmm8
	jb .LBB_18
	pshufd xmm4, xmm6, 85
	movd eax, xmm4
	cvtdq2ps xmm4, xmm4
	movaps xmm5, xmm1
	shufps xmm5, xmm1, 85
	addss xmm5, xmm4
	addss xmm4, xmm4
	mulss xmm4, xmm5
	movlhps xmm5, xmm1
	shufps xmm5, xmm1, 226
	pshufd xmm1, xmm2, 85
	movd ecx, xmm1
	imul eax, eax, -1136930381
	add eax, ecx
	movd xmm6, eax
	punpcklqdq xmm6, xmm2
	shufps xmm6, xmm2, 226
	movaps xmm1, xmm5
	movaps xmm2, xmm6
	subss xmm3, xmm4
	xorps xmm4, xmm4
	ucomiss xmm3, xmm4
	ja .LBB_20
	jmp .LBB_21
.LBB_18:
	pshufd xmm4, xmm6, 238
	movd eax, xmm4
	cvtdq2ps xmm4, xmm4
	movaps xmm5, xmm1
	unpckhpd xmm5, xmm1
	addss xmm5, xmm4
	addss xmm4, xmm4
	mulss xmm4, xmm5
	shufps xmm5, xmm1, 48
	shufps xmm1, xmm5, 132
	pshufd xmm5, xmm2, 238
	movd ecx, xmm5
	imul eax, eax, -1720413743
	add eax, ecx
	movd xmm5, eax
	shufps xmm5, xmm2, 48
	shufps xmm2, xmm5, 132
	subss xmm3, xmm4
	xorps xmm4, xmm4
	ucomiss xmm3, xmm4
	jbe .LBB_21
.LBB_20:
	mulss xmm3, xmm3
	pshufd xmm4, xmm2, 85
	pshufd xmm5, xmm2, 238
	pxor xmm5, xmm4
	pxor xmm5, xmm2
	movd eax, xmm5
	not eax
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	mulps xmm1, xmmword ptr [rax + 4*rcx]
	mulss xmm3, xmm3
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 85
	addss xmm2, xmm1
	movhlps xmm1, xmm1
	addss xmm1, xmm2
	mulss xmm1, xmm3
	addss xmm0, xmm1
.LBB_21:
	mulss xmm0, dword ptr [rip + .LCPI16_12]
	ret