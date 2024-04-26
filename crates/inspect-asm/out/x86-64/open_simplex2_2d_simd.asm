inspect_asm::open_simplex2_2d_simd:
	movsd xmm0, qword ptr [rdi]
	movaps xmm1, xmm0
	shufps xmm1, xmm0, 85
	addss xmm1, xmm0
	mulss xmm1, dword ptr [rip + .LCPI15_0]
	shufps xmm1, xmm1, 0
	addps xmm1, xmm0
	cvttps2dq xmm0, xmm1
	xorps xmm2, xmm2
	cmpnleps xmm2, xmm1
	paddd xmm2, xmm0
	cvtdq2ps xmm0, xmm2
	subps xmm1, xmm0
	movaps xmm6, xmm1
	shufps xmm6, xmm1, 85
	addss xmm6, xmm1
	mulss xmm6, dword ptr [rip + .LCPI15_1]
	movaps xmm0, xmm6
	shufps xmm0, xmm6, 0
	subps xmm1, xmm0
	pshufd xmm0, xmm2, 245
	pmuludq xmm2, xmmword ptr [rip + .LCPI15_2]
	pshufd xmm4, xmm2, 232
	pmuludq xmm0, xmmword ptr [rip + .LCPI15_3]
	pshufd xmm0, xmm0, 232
	punpckldq xmm4, xmm0
	movaps xmm0, xmm1
	mulps xmm0, xmm1
	movss xmm2, dword ptr [rip + .LCPI15_4]
	movaps xmm5, xmm2
	subss xmm5, xmm0
	shufps xmm0, xmm0, 85
	subss xmm5, xmm0
	xorps xmm0, xmm0
	ucomiss xmm0, xmm5
	xorps xmm3, xmm3
	jae .LBB_2
	movaps xmm7, xmm5
	mulss xmm7, xmm5
	mulss xmm7, xmm7
	pshufd xmm3, xmm4, 85
	pxor xmm3, xmm4
	movd eax, xmm3
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 254
	lea rax, [rip + .L__unnamed__0]
	movsd xmm8, qword ptr [rax + 4*rcx]
	mulps xmm8, xmm1
	movaps xmm3, xmm8
	shufps xmm3, xmm8, 85
	addss xmm3, xmm8
	mulss xmm3, xmm7
.LBB_2:
	mulss xmm6, dword ptr [rip + .LCPI15_5]
	addss xmm5, dword ptr [rip + .LCPI15_6]
	addss xmm5, xmm6
	ucomiss xmm0, xmm5
	jae .LBB_4
	movaps xmm0, xmmword ptr [rip + .LCPI15_7]
	addps xmm0, xmm1
	mulss xmm5, xmm5
	mulss xmm5, xmm5
	movdqa xmm6, xmmword ptr [rip + .LCPI15_2]
	paddd xmm6, xmm4
	pshufd xmm7, xmm6, 85
	pxor xmm7, xmm6
	movd eax, xmm7
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 254
	lea rax, [rip + .L__unnamed__0]
	movsd xmm6, qword ptr [rax + 4*rcx]
	mulps xmm6, xmm0
	movaps xmm0, xmm6
	shufps xmm0, xmm6, 85
	addss xmm0, xmm6
	mulss xmm0, xmm5
.LBB_4:
	movaps xmm5, xmm1
	shufps xmm5, xmm1, 85
	ucomiss xmm5, xmm1
	jbe .LBB_5
	addps xmm1, xmmword ptr [rip + .LCPI15_9]
	movaps xmm5, xmm1
	mulps xmm5, xmm1
	subss xmm2, xmm5
	shufps xmm5, xmm5, 85
	subss xmm2, xmm5
	xorps xmm5, xmm5
	ucomiss xmm5, xmm2
	jae .LBB_10
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	movd ecx, xmm4
	pshufd xmm4, xmm4, 85
	movd eax, xmm4
	add eax, 1136930381
	jmp .LBB_7
.LBB_5:
	addps xmm1, xmmword ptr [rip + .LCPI15_8]
	movaps xmm5, xmm1
	mulps xmm5, xmm1
	subss xmm2, xmm5
	shufps xmm5, xmm5, 85
	subss xmm2, xmm5
	xorps xmm5, xmm5
	ucomiss xmm5, xmm2
	jae .LBB_10
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	movd ecx, xmm4
	add ecx, 501125321
	pshufd xmm4, xmm4, 85
	movd eax, xmm4
.LBB_7:
	xor eax, ecx
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 254
	lea rax, [rip + .L__unnamed__0]
	movsd xmm4, qword ptr [rax + 4*rcx]
	mulps xmm4, xmm1
	movaps xmm5, xmm4
	shufps xmm5, xmm4, 85
	addss xmm5, xmm4
	mulss xmm5, xmm2
.LBB_10:
	addss xmm3, xmm5
	addss xmm0, xmm3
	mulss xmm0, dword ptr [rip + .LCPI15_10]
	ret