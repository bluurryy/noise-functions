inspect_asm::open_simplex2s_2d:
	movq xmm0, rdi
	pshufd xmm4, xmm0, 85
	addss xmm4, xmm0
	mulss xmm4, dword ptr [rip + .LCPI17_0]
	shufps xmm4, xmm4, 0
	addps xmm4, xmm0
	movaps xmm1, xmm4
	shufps xmm1, xmm4, 255
	cvttss2si ecx, xmm1
	movss xmm0, dword ptr [rip + .LCPI17_1]
	ucomiss xmm1, xmm0
	mov edx, 2147483647
	cmova ecx, edx
	xor eax, eax
	ucomiss xmm1, xmm1
	cmovp ecx, eax
	movaps xmm1, xmm4
	unpckhpd xmm1, xmm4
	cvttss2si esi, xmm1
	ucomiss xmm1, xmm0
	cmova esi, edx
	ucomiss xmm1, xmm1
	cmovp esi, eax
	cvttss2si edi, xmm4
	ucomiss xmm4, xmm0
	cmova edi, edx
	xorps xmm1, xmm1
	ucomiss xmm4, xmm4
	cmovp edi, eax
	movaps xmm2, xmm4
	shufps xmm2, xmm4, 85
	cvttss2si r8d, xmm2
	ucomiss xmm2, xmm0
	movd xmm0, ecx
	cmova r8d, edx
	movd xmm3, esi
	punpckldq xmm3, xmm0
	ucomiss xmm2, xmm2
	movd xmm0, edi
	cmovp r8d, eax
	movd xmm2, r8d
	punpckldq xmm0, xmm2
	punpcklqdq xmm0, xmm3
	cmpnleps xmm1, xmm4
	paddd xmm1, xmm0
	cvtdq2ps xmm0, xmm1
	subps xmm4, xmm0
	movd eax, xmm1
	imul ecx, eax, 501125321
	pshufd xmm0, xmm1, 85
	movd eax, xmm0
	imul eax, eax, 1136930381
	lea edi, [rcx + 501125321]
	lea esi, [rax + 1136930381]
	movaps xmm3, xmm4
	shufps xmm3, xmm4, 85
	movaps xmm0, xmm4
	addss xmm0, xmm3
	movss xmm1, dword ptr [rip + .LCPI17_2]
	mulss xmm0, xmm1
	mov edx, eax
	xor edx, ecx
	imul edx, edx, 668265261
	mov r8d, edx
	shr r8d, 15
	xor r8d, edx
	and r8d, 254
	movss xmm5, dword ptr [rip + .LCPI17_4]
	mov edx, esi
	xor edx, edi
	imul edx, edx, 668265261
	mov r9d, edx
	shr r9d, 15
	xor r9d, edx
	and r9d, 254
	ucomiss xmm0, xmm1
	mulss xmm5, xmm0
	shufps xmm0, xmm0, 0
	movaps xmm1, xmm4
	subps xmm1, xmm0
	movaps xmm0, xmm1
	shufps xmm0, xmm1, 85
	movaps xmm6, xmm1
	mulss xmm6, xmm1
	movss xmm2, dword ptr [rip + .LCPI17_3]
	movaps xmm7, xmm2
	subss xmm7, xmm6
	movaps xmm6, xmm0
	mulss xmm6, xmm0
	subss xmm7, xmm6
	movaps xmm6, xmm7
	mulss xmm6, xmm7
	mulss xmm6, xmm6
	lea rdx, [rip + .L__unnamed__0]
	movss xmm8, dword ptr [rdx + 4*r8]
	mulss xmm8, xmm1
	movss xmm9, dword ptr [rdx + 4*r8 + 4]
	mulss xmm9, xmm0
	addss xmm9, xmm8
	mulss xmm9, xmm6
	addss xmm7, dword ptr [rip + .LCPI17_5]
	addss xmm7, xmm5
	movss xmm5, dword ptr [rip + .LCPI17_6]
	movaps xmm6, xmm1
	addss xmm6, xmm5
	addss xmm0, xmm5
	mulss xmm7, xmm7
	mulss xmm7, xmm7
	mulss xmm6, dword ptr [rdx + 4*r9]
	mulss xmm0, dword ptr [rdx + 4*r9 + 4]
	addss xmm0, xmm6
	mulss xmm0, xmm7
	addss xmm0, xmm9
	movaps xmm5, xmm4
	subss xmm5, xmm3
	addss xmm4, xmm5
	jbe .LBB_1
	ucomiss xmm4, dword ptr [rip + .LCPI17_11]
	jbe .LBB_12
	movaps xmm4, xmmword ptr [rip + .LCPI17_12]
	addps xmm4, xmm1
	movaps xmm7, xmm4
	mulps xmm7, xmm4
	movaps xmm6, xmm2
	subss xmm6, xmm7
	shufps xmm7, xmm7, 85
	subss xmm6, xmm7
	xorps xmm7, xmm7
	ucomiss xmm6, xmm7
	jbe .LBB_16
	mulss xmm6, xmm6
	mulss xmm6, xmm6
	add ecx, 1002250642
	jmp .LBB_14
.LBB_1:
	xorps xmm7, xmm7
	ucomiss xmm7, xmm4
	jbe .LBB_2
	movaps xmm4, xmmword ptr [rip + .LCPI17_8]
	addps xmm4, xmm1
	movaps xmm8, xmm4
	mulps xmm8, xmm4
	movaps xmm6, xmm2
	subss xmm6, xmm8
	shufps xmm8, xmm8, 85
	subss xmm6, xmm8
	ucomiss xmm6, xmm7
	jbe .LBB_6
	mulss xmm6, xmm6
	mulss xmm6, xmm6
	lea edi, [rcx - 501125321]
	jmp .LBB_4
.LBB_12:
	movaps xmm4, xmmword ptr [rip + .LCPI17_9]
	addps xmm4, xmm1
	movaps xmm7, xmm4
	mulps xmm7, xmm4
	movaps xmm6, xmm2
	subss xmm6, xmm7
	shufps xmm7, xmm7, 85
	subss xmm6, xmm7
	xorps xmm7, xmm7
	ucomiss xmm6, xmm7
	jbe .LBB_16
	mulss xmm6, xmm6
	mulss xmm6, xmm6
.LBB_14:
	xor esi, ecx
	imul ecx, esi, 668265261
	mov esi, ecx
	shr esi, 15
	xor esi, ecx
	and esi, 254
	movsd xmm7, qword ptr [rdx + 4*rsi]
	mulps xmm7, xmm4
	movaps xmm4, xmm7
	shufps xmm4, xmm7, 85
	addss xmm4, xmm7
	mulss xmm4, xmm6
	addss xmm0, xmm4
.LBB_16:
	subss xmm3, xmm5
	ucomiss xmm3, dword ptr [rip + .LCPI17_11]
	jbe .LBB_17
	addps xmm1, xmmword ptr [rip + .LCPI17_13]
	movaps xmm3, xmm1
	mulps xmm3, xmm1
	subss xmm2, xmm3
	shufps xmm3, xmm3, 85
	subss xmm2, xmm3
	xorps xmm3, xmm3
	ucomiss xmm2, xmm3
	jbe .LBB_24
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	add eax, -2021106534
	jmp .LBB_21
.LBB_2:
	movaps xmm4, xmmword ptr [rip + .LCPI17_7]
	addps xmm4, xmm1
	movaps xmm8, xmm4
	mulps xmm8, xmm4
	movaps xmm6, xmm2
	subss xmm6, xmm8
	shufps xmm8, xmm8, 85
	subss xmm6, xmm8
	ucomiss xmm6, xmm7
	jbe .LBB_6
	mulss xmm6, xmm6
	mulss xmm6, xmm6
.LBB_4:
	xor edi, eax
	imul edi, edi, 668265261
	mov r8d, edi
	shr r8d, 15
	xor r8d, edi
	and r8d, 254
	movsd xmm7, qword ptr [rdx + 4*r8]
	mulps xmm7, xmm4
	movaps xmm4, xmm7
	shufps xmm4, xmm7, 85
	addss xmm4, xmm7
	mulss xmm4, xmm6
	addss xmm0, xmm4
.LBB_6:
	ucomiss xmm5, xmm3
	jbe .LBB_7
	addps xmm1, xmmword ptr [rip + .LCPI17_10]
	movaps xmm3, xmm1
	mulps xmm3, xmm1
	subss xmm2, xmm3
	shufps xmm3, xmm3, 85
	subss xmm2, xmm3
	xorps xmm3, xmm3
	ucomiss xmm2, xmm3
	jbe .LBB_24
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	add eax, -1136930381
	xor eax, ecx
	jmp .LBB_22
.LBB_17:
	addps xmm1, xmmword ptr [rip + .LCPI17_7]
	movaps xmm3, xmm1
	mulps xmm3, xmm1
	subss xmm2, xmm3
	shufps xmm3, xmm3, 85
	subss xmm2, xmm3
	xorps xmm3, xmm3
	ucomiss xmm2, xmm3
	jbe .LBB_24
	mulss xmm2, xmm2
	mulss xmm2, xmm2
.LBB_21:
	xor eax, edi
.LBB_22:
	mov esi, eax
	jmp .LBB_23
.LBB_7:
	addps xmm1, xmmword ptr [rip + .LCPI17_9]
	movaps xmm3, xmm1
	mulps xmm3, xmm1
	subss xmm2, xmm3
	shufps xmm3, xmm3, 85
	subss xmm2, xmm3
	xorps xmm3, xmm3
	ucomiss xmm2, xmm3
	jbe .LBB_24
	mulss xmm2, xmm2
	mulss xmm2, xmm2
	xor esi, ecx
.LBB_23:
	imul eax, esi, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 254
	movsd xmm3, qword ptr [rdx + 4*rcx]
	mulps xmm3, xmm1
	movaps xmm1, xmm3
	shufps xmm1, xmm3, 85
	addss xmm1, xmm3
	mulss xmm1, xmm2
	addss xmm0, xmm1
.LBB_24:
	mulss xmm0, dword ptr [rip + .LCPI17_14]
	ret