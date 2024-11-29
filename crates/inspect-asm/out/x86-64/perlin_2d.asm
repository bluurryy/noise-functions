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
	movaps xmm4, xmm1
	mulss xmm4, xmm1
	mulss xmm4, xmm1
	movss xmm2, dword ptr [rip + .LCPI21_1]
	movaps xmm3, xmm1
	mulss xmm3, xmm2
	movss xmm5, dword ptr [rip + .LCPI21_2]
	addss xmm3, xmm5
	mulss xmm3, xmm1
	movss xmm6, dword ptr [rip + .LCPI21_3]
	addss xmm3, xmm6
	mulss xmm3, xmm4
	movaps xmm4, xmm1
	shufps xmm4, xmm1, 85
	mulss xmm2, xmm4
	addss xmm2, xmm5
	mulss xmm2, xmm4
	addss xmm2, xmm6
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
	movss xmm5, dword ptr [rax + 4*r8]
	movaps xmm0, xmm1
	movss xmm0, xmm5
	mulps xmm0, xmm1
	movss xmm5, dword ptr [rax + 4*r8 + 4]
	mulss xmm5, xmm4
	xor edi, ecx
	imul r8d, edi, 668265261
	mov edi, r8d
	shr edi, 15
	xor edi, r8d
	and edi, 254
	movss xmm8, dword ptr [rax + 4*rdi + 4]
	mulss xmm8, xmm4
	addss xmm5, xmm0
	xor esi, edx
	imul r8d, esi, 668265261
	mov esi, r8d
	shr esi, 15
	xor esi, r8d
	and esi, 254
	movss xmm7, dword ptr [rax + 4*rsi]
	mulss xmm7, xmm1
	xor edx, ecx
	imul ecx, edx, 668265261
	mov edx, ecx
	shr edx, 15
	xor edx, ecx
	and edx, 254
	addps xmm1, xmmword ptr [rip + .LCPI21_4]
	movss xmm6, dword ptr [rax + 4*rdi]
	mulss xmm6, xmm1
	addss xmm6, xmm8
	subss xmm6, xmm5
	mulss xmm6, xmm3
	movsd xmm8, qword ptr [rax + 4*rdx]
	mulps xmm8, xmm1
	shufps xmm1, xmm1, 85
	mulss xmm1, dword ptr [rax + 4*rsi + 4]
	addss xmm1, xmm7
	movaps xmm7, xmm8
	shufps xmm7, xmm8, 85
	addss xmm7, xmm8
	subss xmm7, xmm1
	mulss xmm7, xmm3
	addss xmm7, xmm1
	addss xmm6, xmm5
	subss xmm7, xmm6
	shufps xmm0, xmm0, 85
	mulss xmm0, xmm4
	mulss xmm0, xmm2
	mulss xmm0, xmm7
	addss xmm0, xmm6
	mulss xmm0, dword ptr [rip + .LCPI21_5]
	ret