inspect_asm::perlin_2d:
	movq xmm1, rdi
	cvttss2si eax, xmm1
	movss xmm2, dword ptr [rip + .LCPI21_0]
	ucomiss xmm1, xmm2
	mov ecx, 2147483647
	cmova eax, ecx
	xor edx, edx
	ucomiss xmm1, xmm1
	cmovp eax, edx
	pshufd xmm3, xmm1, 85
	cvttss2si esi, xmm3
	ucomiss xmm3, xmm2
	cmova esi, ecx
	xorps xmm0, xmm0
	movd xmm4, eax
	ucomiss xmm3, xmm3
	cmovp esi, edx
	ucomiss xmm0, xmm2
	movd xmm2, esi
	cmova eax, ecx
	ucomiss xmm0, xmm0
	cmovp eax, edx
	punpckldq xmm4, xmm2
	movd xmm2, eax
	pshufd xmm2, xmm2, 0
	punpcklqdq xmm4, xmm2
	cmpnleps xmm0, xmm1
	paddd xmm0, xmm4
	cvtdq2ps xmm2, xmm0
	subps xmm1, xmm2
	movss xmm2, dword ptr [rip + .LCPI21_1]
	movaps xmm3, xmm1
	mulss xmm3, xmm2
	movss xmm4, dword ptr [rip + .LCPI21_2]
	addss xmm3, xmm4
	mulss xmm3, xmm1
	addss xmm3, dword ptr [rip + .LCPI21_3]
	movlhps xmm2, xmm1
	shufps xmm2, xmm1, 226
	mulps xmm2, xmm1
	unpcklps xmm4, xmm4
	addps xmm4, xmm2
	mulps xmm2, xmm1
	shufps xmm4, xmm2, 1
	shufps xmm4, xmm2, 226
	movaps xmm2, xmm1
	movss xmm2, xmm3
	mulps xmm2, xmm4
	movd eax, xmm0
	imul esi, eax, 501125321
	pshufd xmm0, xmm0, 85
	movd eax, xmm0
	imul edi, eax, 1136930381
	lea ecx, [rsi + 501125321]
	lea edx, [rdi + 1136930381]
	mov eax, edi
	xor eax, esi
	imul eax, eax, 668265261
	mov r8d, eax
	shr r8d, 15
	xor r8d, eax
	and r8d, 254
	lea rax, [rip + .L__unnamed__0]
	movss xmm0, dword ptr [rax + 4*r8]
	movaps xmm5, xmm1
	movss xmm5, xmm0
	mulps xmm5, xmm1
	movaps xmm0, xmm1
	shufps xmm0, xmm1, 85
	movss xmm3, dword ptr [rax + 4*r8 + 4]
	mulss xmm3, xmm0
	addss xmm3, xmm5
	mulps xmm5, xmm1
	xor edi, ecx
	imul r8d, edi, 668265261
	mov edi, r8d
	shr edi, 15
	xor edi, r8d
	and edi, 254
	mulss xmm0, dword ptr [rax + 4*rdi + 4]
	xor esi, edx
	imul r8d, esi, 668265261
	mov esi, r8d
	shr esi, 15
	xor esi, r8d
	and esi, 254
	movss xmm6, dword ptr [rax + 4*rsi]
	mulss xmm6, xmm1
	xor edx, ecx
	imul ecx, edx, 668265261
	mov edx, ecx
	shr edx, 15
	xor edx, ecx
	and edx, 254
	addps xmm1, xmmword ptr [rip + .LCPI21_4]
	movss xmm4, dword ptr [rax + 4*rdi]
	mulss xmm4, xmm1
	addss xmm4, xmm0
	subss xmm4, xmm3
	movaps xmm0, xmmword ptr [rip + .LCPI21_5]
	addps xmm0, xmm2
	mulps xmm0, xmm5
	movsd xmm5, qword ptr [rax + 4*rdx]
	mulps xmm5, xmm1
	shufps xmm1, xmm1, 85
	mulss xmm1, dword ptr [rax + 4*rsi + 4]
	addss xmm1, xmm6
	movaps xmm6, xmm5
	shufps xmm6, xmm5, 85
	addss xmm6, xmm5
	subss xmm6, xmm1
	mulss xmm6, xmm2
	addss xmm6, xmm1
	mulss xmm4, xmm2
	addss xmm4, xmm3
	subss xmm6, xmm4
	shufps xmm0, xmm0, 85
	mulss xmm0, xmm6
	addss xmm0, xmm4
	mulss xmm0, dword ptr [rip + .LCPI21_6]
	ret