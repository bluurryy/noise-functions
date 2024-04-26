inspect_asm::open_simplex2_2d:
	movq xmm0, rdi
	pshufd xmm1, xmm0, 85
	addss xmm1, xmm0
	mulss xmm1, dword ptr [rip + .LCPI13_0]
	shufps xmm1, xmm1, 0
	addps xmm1, xmm0
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 255
	cvttss2si edx, xmm2
	movss xmm0, dword ptr [rip + .LCPI13_1]
	ucomiss xmm2, xmm0
	mov eax, 2147483647
	cmova edx, eax
	xor ecx, ecx
	ucomiss xmm2, xmm2
	cmovp edx, ecx
	movaps xmm2, xmm1
	unpckhpd xmm2, xmm1
	cvttss2si esi, xmm2
	ucomiss xmm2, xmm0
	movd xmm3, edx
	cmova esi, eax
	ucomiss xmm2, xmm2
	cmovp esi, ecx
	cvttss2si edx, xmm1
	ucomiss xmm1, xmm0
	movd xmm2, esi
	cmova edx, eax
	ucomiss xmm1, xmm1
	cmovp edx, ecx
	movaps xmm4, xmm1
	shufps xmm4, xmm1, 85
	ucomiss xmm4, xmm0
	xorps xmm0, xmm0
	movd xmm5, edx
	cvttss2si edx, xmm4
	punpckldq xmm2, xmm3
	cmova edx, eax
	ucomiss xmm4, xmm4
	cmovp edx, ecx
	movd xmm3, edx
	punpckldq xmm5, xmm3
	punpcklqdq xmm5, xmm2
	cmpnleps xmm0, xmm1
	paddd xmm0, xmm5
	cvtdq2ps xmm2, xmm0
	subps xmm1, xmm2
	movaps xmm5, xmm1
	shufps xmm5, xmm1, 85
	addss xmm5, xmm1
	mulss xmm5, dword ptr [rip + .LCPI13_2]
	movaps xmm2, xmm5
	shufps xmm2, xmm5, 0
	subps xmm1, xmm2
	movd eax, xmm0
	imul eax, eax, 501125321
	pshufd xmm0, xmm0, 85
	movd ecx, xmm0
	imul ecx, ecx, 1136930381
	movaps xmm0, xmm1
	mulps xmm0, xmm1
	movss xmm2, dword ptr [rip + .LCPI13_3]
	movaps xmm4, xmm2
	subss xmm4, xmm0
	shufps xmm0, xmm0, 85
	subss xmm4, xmm0
	xorps xmm0, xmm0
	ucomiss xmm0, xmm4
	pxor xmm3, xmm3
	jae .LBB_2
	movaps xmm6, xmm4
	mulss xmm6, xmm4
	mulss xmm6, xmm6
	mov edx, ecx
	xor edx, eax
	imul edx, edx, 668265261
	mov esi, edx
	shr esi, 15
	xor esi, edx
	and esi, 254
	lea rdx, [rip + .L__unnamed__0]
	movsd xmm7, qword ptr [rdx + 4*rsi]
	mulps xmm7, xmm1
	movaps xmm3, xmm7
	shufps xmm3, xmm7, 85
	addss xmm3, xmm7
	mulss xmm3, xmm6
.LBB_2:
	mulss xmm5, dword ptr [rip + .LCPI13_4]
	addss xmm4, dword ptr [rip + .LCPI13_5]
	addss xmm4, xmm5
	ucomiss xmm0, xmm4
	jae .LBB_4
	mulss xmm4, xmm4
	mulss xmm4, xmm4
	lea edx, [rax + 501125321]
	lea esi, [rcx + 1136930381]
	xor esi, edx
	imul edx, esi, 668265261
	mov esi, edx
	shr esi, 15
	xor esi, edx
	and esi, 254
	lea rdx, [rip + .L__unnamed__0]
	movaps xmm0, xmmword ptr [rip + .LCPI13_6]
	addps xmm0, xmm1
	movsd xmm5, qword ptr [rdx + 4*rsi]
	mulps xmm5, xmm0
	movaps xmm0, xmm5
	shufps xmm0, xmm5, 85
	addss xmm0, xmm5
	mulss xmm0, xmm4
.LBB_4:
	movaps xmm4, xmm1
	shufps xmm4, xmm1, 85
	ucomiss xmm4, xmm1
	jbe .LBB_5
	addps xmm1, xmmword ptr [rip + .LCPI13_8]
	movaps xmm4, xmm1
	mulps xmm4, xmm1
	subss xmm2, xmm4
	shufps xmm4, xmm4, 85
	subss xmm2, xmm4
	xorps xmm4, xmm4
	ucomiss xmm4, xmm2
	jae .LBB_10
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	add ecx, 1136930381
	xor ecx, eax
	mov eax, ecx
	jmp .LBB_7
.LBB_5:
	addps xmm1, xmmword ptr [rip + .LCPI13_7]
	movaps xmm4, xmm1
	mulps xmm4, xmm1
	subss xmm2, xmm4
	shufps xmm4, xmm4, 85
	subss xmm2, xmm4
	xorps xmm4, xmm4
	ucomiss xmm4, xmm2
	jae .LBB_10
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	add eax, 501125321
	xor eax, ecx
.LBB_7:
	imul eax, eax, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 254
	lea rax, [rip + .L__unnamed__0]
	movsd xmm5, qword ptr [rax + 4*rcx]
	mulps xmm5, xmm1
	movaps xmm4, xmm5
	shufps xmm4, xmm5, 85
	addss xmm4, xmm5
	mulss xmm4, xmm2
.LBB_10:
	addss xmm3, xmm4
	addss xmm0, xmm3
	mulss xmm0, dword ptr [rip + .LCPI13_9]
	ret