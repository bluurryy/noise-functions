inspect_asm::perlin_2d_simd:
	movsd xmm2, qword ptr [rdi]
	cvttps2dq xmm0, xmm2
	xorps xmm4, xmm4
	cmpnleps xmm4, xmm2
	paddd xmm4, xmm0
	cvtdq2ps xmm0, xmm4
	subps xmm2, xmm0
	movaps xmm0, xmmword ptr [rip + .LCPI23_0]
	addps xmm0, xmm2
	movaps xmm3, xmm2
	mulps xmm3, xmm2
	mulps xmm3, xmm2
	movaps xmm1, xmmword ptr [rip + .LCPI23_1]
	mulps xmm1, xmm2
	addps xmm1, xmmword ptr [rip + .LCPI23_2]
	mulps xmm1, xmm2
	addps xmm1, xmmword ptr [rip + .LCPI23_3]
	mulps xmm1, xmm3
	movq xmm5, qword ptr [rip + .LCPI23_7]
	pshufd xmm6, xmm4, 245
	pmuludq xmm4, xmm5
	pshufd xmm3, xmm4, 232
	pmuludq xmm6, xmmword ptr [rip + .LCPI23_5]
	pshufd xmm7, xmm6, 232
	punpckldq xmm3, xmm7
	paddd xmm3, xmm5
	movd ecx, xmm4
	movd esi, xmm6
	mov eax, ecx
	xor eax, esi
	imul eax, eax, 668265261
	mov edx, eax
	shr edx, 15
	xor edx, eax
	and edx, 254
	lea rax, [rip + .L__unnamed__0]
	movsd xmm4, qword ptr [rax + 4*rdx]
	mulps xmm4, xmm2
	movd edx, xmm3
	xor esi, edx
	imul esi, esi, 668265261
	mov edi, esi
	shr edi, 15
	xor edi, esi
	and edi, 254
	movsd xmm5, qword ptr [rax + 4*rdi]
	movaps xmm6, xmm0
	unpcklps xmm6, xmm2
	movaps xmm7, xmm2
	shufps xmm7, xmm0, 1
	shufps xmm7, xmm0, 226
	mulps xmm7, xmm5
	movaps xmm5, xmm4
	shufps xmm5, xmm4, 85
	addss xmm5, xmm4
	movaps xmm2, xmm7
	shufps xmm2, xmm7, 85
	addss xmm2, xmm7
	subss xmm2, xmm5
	mulss xmm2, xmm1
	addss xmm2, xmm5
	pshufd xmm3, xmm3, 85
	movd esi, xmm3
	xor ecx, esi
	imul ecx, ecx, 668265261
	mov edi, ecx
	shr edi, 15
	xor edi, ecx
	and edi, 254
	xor edx, esi
	imul ecx, edx, 668265261
	mov edx, ecx
	shr edx, 15
	xor edx, ecx
	and edx, 254
	movss xmm3, dword ptr [rax + 4*rdx]
	movss xmm4, dword ptr [rax + 4*rdx + 4]
	movss xmm5, dword ptr [rax + 4*rdi]
	unpcklps xmm3, xmm5
	mulps xmm3, xmm6
	movss xmm5, dword ptr [rax + 4*rdi + 4]
	unpcklps xmm4, xmm5
	shufps xmm0, xmm0, 85
	mulps xmm0, xmm4
	addps xmm0, xmm3
	movaps xmm3, xmm0
	shufps xmm3, xmm0, 85
	subss xmm0, xmm3
	mulss xmm0, xmm1
	addss xmm0, xmm3
	shufps xmm1, xmm1, 85
	subss xmm0, xmm2
	mulss xmm0, xmm1
	addss xmm0, xmm2
	mulss xmm0, dword ptr [rip + .LCPI23_6]
	ret