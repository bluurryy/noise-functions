inspect_asm::perlin_3d:
	push rbp
	push r14
	push rbx
	movss xmm0, dword ptr [rdi + 4]
	cvttss2si eax, xmm0
	movss xmm3, dword ptr [rip + .LCPI22_0]
	ucomiss xmm0, xmm3
	mov esi, 2147483647
	cmova eax, esi
	xor ecx, ecx
	ucomiss xmm0, xmm0
	movss xmm4, dword ptr [rdi]
	cmovp eax, ecx
	xorps xmm1, xmm1
	ucomiss xmm0, xmm1
	sbb eax, 0
	movss xmm8, dword ptr [rdi + 8]
	xorps xmm1, xmm1
	cvtsi2ss xmm1, eax
	subss xmm0, xmm1
	movaps xmm1, xmm0
	mulss xmm1, xmm0
	mulss xmm1, xmm0
	movss xmm6, dword ptr [rip + .LCPI22_1]
	movaps xmm7, xmm0
	mulss xmm7, xmm6
	movss xmm2, dword ptr [rip + .LCPI22_2]
	addss xmm7, xmm2
	mulss xmm7, xmm0
	movss xmm5, dword ptr [rip + .LCPI22_3]
	imul edx, eax, 1136930381
	cvttss2si edi, xmm4
	ucomiss xmm4, xmm3
	addss xmm7, xmm5
	cmova edi, esi
	ucomiss xmm4, xmm4
	cmovp edi, ecx
	cvttss2si r8d, xmm8
	ucomiss xmm8, xmm3
	lea eax, [rdx + 1136930381]
	cmova r8d, esi
	ucomiss xmm8, xmm8
	movaps xmm3, xmm8
	unpcklps xmm3, xmm4
	xorps xmm9, xmm9
	movd xmm4, edi
	cmovp r8d, ecx
	movd xmm8, r8d
	punpckldq xmm8, xmm4
	movq xmm4, xmm8
	cmpnleps xmm9, xmm3
	paddd xmm9, xmm4
	cvtdq2ps xmm4, xmm9
	subps xmm3, xmm4
	movaps xmm8, xmm3
	shufps xmm8, xmm3, 85
	movss xmm4, dword ptr [rip + .LCPI22_4]
	addss xmm4, xmm3
	mulss xmm6, xmm8
	addss xmm6, xmm2
	mulss xmm6, xmm8
	pshufd xmm10, xmm9, 85
	movd ecx, xmm10
	imul esi, ecx, 501125321
	movd ecx, xmm9
	imul r8d, ecx, 1720413743
	lea ebx, [rsi + 501125321]
	mov r9d, edx
	xor r9d, esi
	mov ecx, r8d
	xor ecx, r9d
	imul ecx, ecx, 668265261
	mov r11d, ecx
	shr r11d, 15
	xor r11d, ecx
	and r11d, 252
	lea rcx, [rip + .L__unnamed__0]
	xor esi, eax
	mov edi, esi
	xor edi, r8d
	imul edi, edi, 668265261
	mov r14d, edi
	shr r14d, 15
	xor r14d, edi
	and r14d, 252
	movss xmm9, dword ptr [rcx + 4*r11]
	movss xmm11, dword ptr [rcx + 4*r14]
	unpcklps xmm11, xmm9
	mulps xmm11, xmm8
	mulss xmm8, xmm8
	addss xmm6, xmm5
	shufps xmm1, xmm3, 0
	movaps xmm9, xmm1
	shufps xmm9, xmm3, 226
	movsd xmm1, xmm7
	shufps xmm1, xmm3, 226
	mulps xmm1, xmm9
	movaps xmm7, xmmword ptr [rip + .LCPI22_5]
	unpcklps xmm7, xmm8
	mulps xmm7, xmm3
	movaps xmm8, xmm7
	addps xmm8, xmm2
	unpcklps xmm2, xmm6
	mulps xmm2, xmm7
	movaps xmm10, xmm2
	shufps xmm10, xmm8, 1
	shufps xmm10, xmm8, 226
	lea r10d, [r8 + 1720413743]
	xor edx, ebx
	mov edi, r8d
	xor edi, edx
	imul ebp, edi, 668265261
	mov edi, ebp
	shr edi, 15
	xor edi, ebp
	and edi, 252
	shufps xmm2, xmm2, 85
	xor eax, ebx
	xor r8d, eax
	imul ebx, r8d, 668265261
	mov r8d, ebx
	shr r8d, 15
	xor r8d, ebx
	and r8d, 252
	movaps xmm9, xmm3
	movss xmm9, xmm0
	addps xmm9, xmmword ptr [rip + .LCPI22_6]
	movss xmm6, dword ptr [rcx + 4*r11 + 4]
	movss xmm7, dword ptr [rcx + 4*r11 + 8]
	movss xmm8, dword ptr [rcx + 4*r14 + 4]
	unpcklps xmm8, xmm6
	movss xmm6, dword ptr [rcx + 4*r14 + 8]
	unpcklps xmm6, xmm7
	movaps xmm7, xmm0
	movlhps xmm7, xmm9
	shufps xmm7, xmm9, 226
	mulps xmm7, xmm8
	addps xmm7, xmm11
	movss xmm11, dword ptr [rcx + 4*rdi]
	movss xmm8, dword ptr [rcx + 4*r8 + 4]
	unpcklps xmm8, xmm11
	xor r9d, r10d
	imul r9d, r9d, 668265261
	mov r11d, r9d
	shr r11d, 15
	xor r11d, r9d
	and r11d, 252
	xor edx, r10d
	imul edx, edx, 668265261
	mov r9d, edx
	shr r9d, 15
	xor r9d, edx
	and r9d, 252
	movss xmm11, dword ptr [rcx + 4*r11]
	movss xmm12, dword ptr [rcx + 4*r11 + 4]
	movss xmm13, dword ptr [rcx + 4*r11 + 8]
	movss xmm14, dword ptr [rcx + 4*r9]
	unpcklps xmm14, xmm11
	movss xmm15, dword ptr [rcx + 4*r9 + 4]
	unpcklps xmm15, xmm12
	movss xmm11, dword ptr [rcx + 4*r9 + 8]
	unpcklps xmm11, xmm13
	movaps xmm12, xmm0
	shufps xmm12, xmm9, 212
	shufps xmm12, xmm9, 226
	mulps xmm8, xmm9
	shufps xmm0, xmm0, 0
	mulps xmm0, xmm15
	movaps xmm13, xmm3
	shufps xmm13, xmm9, 17
	shufps xmm13, xmm9, 226
	mulps xmm14, xmm13
	addps xmm0, xmm14
	shufps xmm4, xmm4, 0
	mulps xmm11, xmm4
	addps xmm11, xmm0
	movaps xmm0, xmm11
	shufps xmm0, xmm11, 85
	movaps xmm14, xmm11
	subps xmm14, xmm0
	movaps xmm0, xmm3
	unpcklps xmm0, xmm14
	mulps xmm0, xmm10
	movss xmm11, xmm5
	addps xmm11, xmm0
	xor esi, r10d
	imul edx, esi, 668265261
	mov esi, edx
	shr esi, 15
	xor esi, edx
	and esi, 252
	xor eax, r10d
	imul eax, eax, 668265261
	mov edx, eax
	shr edx, 15
	xor edx, eax
	and edx, 252
	movss xmm0, dword ptr [rcx + 4*rsi]
	movss xmm5, dword ptr [rcx + 4*rsi + 4]
	movss xmm10, dword ptr [rcx + 4*rsi + 8]
	movss xmm14, dword ptr [rcx + 4*rdx]
	unpcklps xmm14, xmm0
	movss xmm15, dword ptr [rcx + 4*rdx + 4]
	unpcklps xmm15, xmm5
	movss xmm0, dword ptr [rcx + 4*rdx + 8]
	unpcklps xmm0, xmm10
	mulps xmm14, xmm13
	shufps xmm9, xmm9, 0
	mulps xmm9, xmm15
	addps xmm9, xmm14
	mulps xmm0, xmm4
	addps xmm0, xmm9
	movaps xmm4, xmm0
	shufps xmm4, xmm0, 85
	subss xmm0, xmm4
	mulss xmm0, xmm2
	addss xmm0, xmm4
	movaps xmm4, xmm11
	shufps xmm4, xmm11, 85
	subss xmm0, xmm4
	movlhps xmm0, xmm3
	shufps xmm0, xmm3, 226
	shufps xmm3, xmm3, 0
	mulps xmm6, xmm3
	addps xmm6, xmm7
	movss xmm4, dword ptr [rcx + 4*rdi + 4]
	movss xmm5, dword ptr [rcx + 4*rdi + 8]
	movss xmm7, dword ptr [rcx + 4*r8]
	unpcklps xmm7, xmm4
	mulps xmm7, xmm12
	movss xmm4, dword ptr [rcx + 4*r8 + 8]
	unpcklps xmm4, xmm5
	addps xmm8, xmm7
	mulps xmm4, xmm3
	addps xmm4, xmm8
	subps xmm4, xmm6
	mulps xmm4, xmm2
	addps xmm4, xmm6
	movaps xmm2, xmm4
	shufps xmm2, xmm4, 85
	subss xmm4, xmm2
	mulps xmm0, xmm1
	shufps xmm1, xmm1, 85
	mulss xmm1, xmm4
	addss xmm1, xmm2
	movaps xmm2, xmm0
	addps xmm2, xmm11
	shufps xmm2, xmm2, 85
	subss xmm2, xmm1
	mulss xmm0, xmm11
	mulss xmm0, xmm2
	addss xmm0, xmm1
	mulss xmm0, dword ptr [rip + .LCPI22_7]
	pop rbx
	pop r14
	pop rbp
	ret