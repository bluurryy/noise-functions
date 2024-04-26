inspect_asm::perlin_2d_simd:
	movsd xmm2, qword ptr [rdi]
	cvttps2dq xmm0, xmm2
	xorps xmm3, xmm3
	cmpnleps xmm3, xmm2
	paddd xmm3, xmm0
	cvtdq2ps xmm0, xmm3
	subps xmm2, xmm0
	movaps xmm0, xmmword ptr [rip + .LCPI23_0]
	addps xmm0, xmm2
	movaps xmm4, xmm2
	mulps xmm4, xmm2
	movaps xmm1, xmmword ptr [rip + .LCPI23_1]
	mulps xmm1, xmm2
	addps xmm1, xmmword ptr [rip + .LCPI23_2]
	mulps xmm4, xmm2
	mulps xmm1, xmm2
	addps xmm1, xmmword ptr [rip + .LCPI23_3]
	mulps xmm1, xmm4
	movdqa xmm5, xmmword ptr [rip + .LCPI23_4]
	pshufd xmm6, xmm3, 245
	pmuludq xmm3, xmm5
	pshufd xmm4, xmm3, 232
	pmuludq xmm6, xmmword ptr [rip + .LCPI23_5]
	pshufd xmm7, xmm6, 232
	punpckldq xmm4, xmm7
	paddd xmm4, xmm5
	movd ecx, xmm3
	movd esi, xmm6
	mov eax, ecx
	xor eax, esi
	imul eax, eax, 668265261
	mov edx, eax
	shr edx, 15
	xor edx, eax
	and edx, 254
	lea rax, [rip + .L__unnamed__0]
	movsd xmm3, qword ptr [rax + 4*rdx]
	mulps xmm3, xmm2
	movd edx, xmm4
	xor esi, edx
	imul esi, esi, 668265261
	mov edi, esi
	shr edi, 15
	xor edi, esi
	and edi, 254
	movsd xmm5, qword ptr [rax + 4*rdi]
	movaps xmm6, xmmword ptr [rip + .LCPI23_6]
	addps xmm6, xmm2
	mulps xmm6, xmm5
	movaps xmm5, xmm3
	shufps xmm5, xmm3, 85
	addss xmm5, xmm3
	movaps xmm3, xmm6
	shufps xmm3, xmm6, 85
	addss xmm3, xmm6
	subss xmm3, xmm5
	mulss xmm3, xmm1
	addss xmm3, xmm5
	pshufd xmm4, xmm4, 85
	movd esi, xmm4
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
	movaps xmm4, xmm0
	unpcklps xmm4, xmm2
	movss xmm2, dword ptr [rax + 4*rdx]
	movss xmm5, dword ptr [rax + 4*rdx + 4]
	movss xmm6, dword ptr [rax + 4*rdi]
	unpcklps xmm2, xmm6
	mulps xmm2, xmm4
	movss xmm4, dword ptr [rax + 4*rdi + 4]
	unpcklps xmm5, xmm4
	shufps xmm0, xmm0, 85
	mulps xmm0, xmm5
	addps xmm0, xmm2
	movaps xmm2, xmm0
	shufps xmm2, xmm0, 85
	subss xmm0, xmm2
	mulss xmm0, xmm1
	addss xmm0, xmm2
	shufps xmm1, xmm1, 85
	subss xmm0, xmm3
	mulss xmm0, xmm1
	addss xmm0, xmm3
	mulss xmm0, dword ptr [rip + .LCPI23_7]
	ret