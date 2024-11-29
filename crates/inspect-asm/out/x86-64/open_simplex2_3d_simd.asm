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
	cvtdq2ps xmm3, xmm4
	movaps xmm7, xmmword ptr [rip + .LCPI16_6]
	xorps xmm7, xmm5
	mulps xmm7, xmm3
	pshufd xmm1, xmm0, 245
	pmuludq xmm0, xmmword ptr [rip + .LCPI16_7]
	pmuludq xmm1, xmmword ptr [rip + .LCPI16_8]
	pshufd xmm6, xmm0, 232
	pshufd xmm0, xmm1, 232
	punpckldq xmm6, xmm0
	movaps xmm0, xmm5
	mulss xmm0, xmm5
	movss xmm1, dword ptr [rip + .LCPI16_9]
	subss xmm1, xmm0
	movaps xmm10, xmm5
	shufps xmm10, xmm5, 85
	movaps xmm0, xmm10
	mulss xmm0, xmm10
	movaps xmm8, xmm5
	unpckhpd xmm8, xmm5
	movaps xmm9, xmm8
	mulss xmm9, xmm8
	addss xmm9, xmm0
	subss xmm1, xmm9
	xorps xmm0, xmm0
	ucomiss xmm1, xmm0
	jbe .LBB_2
	movaps xmm0, xmm1
	mulss xmm0, xmm1
	mulss xmm0, xmm0
	pshufd xmm9, xmm6, 85
	pshufd xmm11, xmm6, 238
	pxor xmm11, xmm9
	pxor xmm11, xmm6
	movd eax, xmm11
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	movaps xmm9, xmmword ptr [rax + 4*rcx]
	mulps xmm9, xmm5
	movaps xmm11, xmm9
	shufps xmm11, xmm9, 85
	addss xmm11, xmm9
	movhlps xmm9, xmm9
	addss xmm9, xmm11
	mulss xmm9, xmm0
	xorps xmm0, xmm0
	addss xmm0, xmm9
.LBB_2:
	movss xmm9, dword ptr [rip + .LCPI16_10]
	addss xmm9, xmm1
	movaps xmm11, xmm7
	shufps xmm11, xmm7, 85
	ucomiss xmm7, xmm11
	movaps xmm12, xmm7
	unpckhpd xmm12, xmm7
	jb .LBB_5
	ucomiss xmm7, xmm12
	jb .LBB_5
	xor eax, eax
	mov ecx, -501125321
	movaps xmm8, xmm5
	jmp .LBB_8
.LBB_5:
	cmpnleps xmm12, xmm11
	movaps xmm13, xmm7
	cmpnltps xmm13, xmm11
	orps xmm13, xmm12
	movd ecx, xmm13
	mov eax, ecx
	and eax, 1
	inc rax
	test cl, 1
	jne .LBB_7
	movaps xmm8, xmm10
.LBB_7:
	mov edx, -1720413743
	mov ecx, -1136930381
	cmovne ecx, edx
.LBB_8:
	movaps xmmword ptr [rsp - 24], xmm4
	mov edx, dword ptr [rsp + 4*rax - 24]
	xorps xmm10, xmm10
	cvtsi2ss xmm10, edx
	addss xmm8, xmm10
	addss xmm10, xmm10
	mulss xmm10, xmm8
	subss xmm9, xmm10
	xorps xmm10, xmm10
	ucomiss xmm9, xmm10
	jbe .LBB_10
	imul ecx, edx
	mov edx, eax
	and edx, 3
	movdqa xmmword ptr [rsp - 104], xmm6
	add ecx, dword ptr [rsp + 4*rax - 104]
	shl edx, 2
	mov dword ptr [rsp + rdx - 104], ecx
	movdqa xmm11, xmmword ptr [rsp - 104]
	movaps xmmword ptr [rsp - 72], xmm5
	movss dword ptr [rsp + rdx - 72], xmm8
	movaps xmm5, xmmword ptr [rsp - 72]
	mulss xmm9, xmm9
	pshufd xmm8, xmm11, 85
	pshufd xmm12, xmm11, 238
	pxor xmm12, xmm8
	pxor xmm12, xmm11
	movd eax, xmm12
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	mulps xmm5, xmmword ptr [rax + 4*rcx]
	mulss xmm9, xmm9
	movaps xmm8, xmm5
	shufps xmm8, xmm5, 85
	addss xmm8, xmm5
	movhlps xmm5, xmm5
	addss xmm5, xmm8
	mulss xmm5, xmm9
	addss xmm0, xmm5
.LBB_10:
	movaps xmm5, xmmword ptr [rip + .LCPI16_2]
	subps xmm5, xmm7
	mulps xmm3, xmm5
	movss xmm9, dword ptr [rip + .LCPI16_11]
	subss xmm9, xmm5
	movaps xmm7, xmm5
	shufps xmm7, xmm5, 85
	movaps xmm8, xmm5
	unpckhpd xmm8, xmm5
	movaps xmm11, xmm7
	addss xmm11, xmm8
	subss xmm9, xmm11
	addss xmm1, xmm9
	psrld xmm2, 1
	pand xmm2, xmmword ptr [rip + .LCPI16_7]
	paddd xmm2, xmm6
	pxor xmm6, xmm6
	ucomiss xmm1, xmm10
	jbe .LBB_12
	movaps xmm9, xmm1
	mulss xmm9, xmm1
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
	mulps xmm10, xmm3
	movaps xmm11, xmm10
	shufps xmm11, xmm10, 85
	addss xmm11, xmm10
	movhlps xmm10, xmm10
	addss xmm10, xmm11
	mulss xmm10, xmm9
	addss xmm0, xmm10
.LBB_12:
	psubd xmm6, xmm4
	addss xmm1, dword ptr [rip + .LCPI16_10]
	ucomiss xmm5, xmm7
	jb .LBB_15
	ucomiss xmm5, xmm8
	jb .LBB_15
	xor eax, eax
	mov ecx, -501125321
	jmp .LBB_16
.LBB_15:
	cmpnleps xmm8, xmm7
	cmpnltps xmm5, xmm7
	orps xmm5, xmm8
	movd ecx, xmm5
	mov eax, ecx
	and eax, 1
	inc rax
	test cl, 1
	mov edx, -1720413743
	mov ecx, -1136930381
	cmovne ecx, edx
.LBB_16:
	movdqa xmmword ptr [rsp - 40], xmm6
	lea esi, [4*rax]
	mov edx, dword ptr [rsp + rsi - 40]
	xorps xmm5, xmm5
	cvtsi2ss xmm5, edx
	movaps xmmword ptr [rsp - 56], xmm3
	movss xmm4, dword ptr [rsp + rsi - 56]
	addss xmm4, xmm5
	addss xmm5, xmm5
	mulss xmm5, xmm4
	subss xmm1, xmm5
	xorps xmm5, xmm5
	ucomiss xmm1, xmm5
	jbe .LBB_18
	imul edx, ecx
	mov ecx, eax
	and ecx, 3
	shl ecx, 2
	movdqa xmmword ptr [rsp - 120], xmm2
	add edx, dword ptr [rsp + 4*rax - 120]
	mov dword ptr [rsp + rcx - 120], edx
	movdqa xmm5, xmmword ptr [rsp - 120]
	movaps xmmword ptr [rsp - 88], xmm3
	movss dword ptr [rsp + rcx - 88], xmm4
	movaps xmm2, xmmword ptr [rsp - 88]
	mulss xmm1, xmm1
	pshufd xmm3, xmm5, 85
	pshufd xmm4, xmm5, 238
	pxor xmm4, xmm3
	pxor xmm4, xmm5
	movd eax, xmm4
	not eax
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	mulps xmm2, xmmword ptr [rax + 4*rcx]
	mulss xmm1, xmm1
	movaps xmm3, xmm2
	shufps xmm3, xmm2, 85
	addss xmm3, xmm2
	movhlps xmm2, xmm2
	addss xmm2, xmm3
	mulss xmm2, xmm1
	addss xmm0, xmm2
.LBB_18:
	mulss xmm0, dword ptr [rip + .LCPI16_12]
	ret