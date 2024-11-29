inspect_asm::perlin_3d:
	push rbp
	push r14
	push rbx
	movss xmm5, dword ptr [rdi]
	cvttss2si eax, xmm5
	movss xmm8, dword ptr [rip + .LCPI22_0]
	ucomiss xmm5, xmm8
	mov edx, 2147483647
	cmova eax, edx
	xor ecx, ecx
	ucomiss xmm5, xmm5
	cmovp eax, ecx
	xorps xmm0, xmm0
	ucomiss xmm5, xmm0
	sbb eax, 0
	xorps xmm0, xmm0
	cvtsi2ss xmm0, eax
	subss xmm5, xmm0
	movss xmm3, dword ptr [rip + .LCPI22_1]
	addss xmm3, xmm5
	movaps xmm6, xmm5
	mulss xmm6, xmm5
	mulss xmm6, xmm5
	movss xmm1, dword ptr [rip + .LCPI22_2]
	movaps xmm0, xmm5
	mulss xmm0, xmm1
	movss xmm7, dword ptr [rip + .LCPI22_3]
	imul eax, eax, 501125321
	ucomiss xmm0, xmm8
	addss xmm0, xmm7
	cmova esi, edx
	mulss xmm0, xmm5
	movsd xmm2, qword ptr [rdi + 4]
	ucomiss xmm0, xmm0
	cmovp esi, ecx
	movaps xmm9, xmm2
	shufps xmm9, xmm2, 255
	cvttss2si edi, xmm9
	ucomiss xmm9, xmm8
	movss xmm4, dword ptr [rip + .LCPI22_4]
	cmova edi, edx
	ucomiss xmm9, xmm9
	cmovp edi, ecx
	cvttss2si r8d, xmm2
	ucomiss xmm2, xmm8
	addss xmm0, xmm4
	cmova r8d, edx
	lea r11d, [rax + 501125321]
	xorps xmm9, xmm9
	ucomiss xmm2, xmm2
	cmovp r8d, ecx
	movaps xmm10, xmm2
	shufps xmm10, xmm2, 85
	cvttss2si r9d, xmm10
	ucomiss xmm10, xmm8
	movd xmm8, esi
	cmova r9d, edx
	movd xmm11, edi
	punpckldq xmm8, xmm11
	ucomiss xmm10, xmm10
	movd xmm10, r8d
	cmovp r9d, ecx
	movd xmm11, r9d
	punpckldq xmm10, xmm11
	punpcklqdq xmm10, xmm8
	cmpnleps xmm9, xmm2
	paddd xmm9, xmm10
	cvtdq2ps xmm8, xmm9
	subps xmm2, xmm8
	shufps xmm6, xmm2, 0
	movaps xmm8, xmm6
	shufps xmm8, xmm2, 226
	movsd xmm6, xmm0
	shufps xmm6, xmm2, 226
	mulps xmm6, xmm8
	movaps xmm0, xmm2
	mulss xmm0, xmm1
	addss xmm0, xmm7
	movaps xmm8, xmm2
	shufps xmm8, xmm2, 85
	movaps xmm10, xmm8
	mulss xmm10, xmm8
	mulss xmm10, xmm8
	mulss xmm1, xmm8
	addss xmm1, xmm7
	mulss xmm1, xmm8
	addss xmm1, xmm4
	mulss xmm1, xmm10
	movd ecx, xmm9
	imul r9d, ecx, 1136930381
	pshufd xmm7, xmm9, 85
	movd ecx, xmm7
	imul ebx, ecx, 1720413743
	lea edi, [r9 + 1136930381]
	lea r8d, [rbx + 1720413743]
	mov r10d, r9d
	xor r10d, eax
	mov ecx, ebx
	xor ecx, r10d
	imul ecx, ecx, 668265261
	mov esi, ecx
	shr esi, 15
	xor esi, ecx
	and esi, 252
	lea rcx, [rip + .L__unnamed__0]
	xor r9d, r11d
	mov edx, ebx
	xor edx, r9d
	imul ebp, edx, 668265261
	mov edx, ebp
	shr edx, 15
	xor edx, ebp
	and edx, 252
	xor eax, edi
	mov ebp, eax
	xor ebp, ebx
	imul ebp, ebp, 668265261
	mov r14d, ebp
	shr r14d, 15
	xor r14d, ebp
	and r14d, 252
	movsd xmm9, qword ptr [rcx + 4*r14]
	movss xmm7, dword ptr [rcx + 4*r14 + 8]
	unpcklps xmm0, xmm7
	mulps xmm0, xmm2
	xor edi, r11d
	xor ebx, edi
	imul ebx, ebx, 668265261
	mov r11d, ebx
	shr r11d, 15
	xor r11d, ebx
	and r11d, 252
	movss xmm10, dword ptr [rcx + 4*r11]
	mulss xmm10, xmm3
	movss xmm11, dword ptr [rcx + 4*r11 + 8]
	mulss xmm11, xmm8
	movaps xmm7, xmmword ptr [rip + .LCPI22_5]
	addps xmm7, xmm2
	movaps xmm8, xmm5
	unpcklps xmm8, xmm7
	mulps xmm8, xmm9
	movaps xmm9, xmm8
	shufps xmm9, xmm8, 85
	addps xmm9, xmm8
	unpcklps xmm4, xmm9
	addps xmm4, xmm0
	movss xmm0, dword ptr [rcx + 4*r11 + 4]
	mulss xmm0, xmm7
	addss xmm0, xmm10
	addss xmm0, xmm11
	movaps xmm8, xmm4
	shufps xmm8, xmm4, 85
	subss xmm0, xmm8
	movlhps xmm0, xmm2
	shufps xmm0, xmm2, 226
	mulps xmm0, xmm6
	shufps xmm6, xmm6, 85
	xor r10d, r8d
	imul r11d, r10d, 668265261
	mov r10d, r11d
	shr r10d, 15
	xor r10d, r11d
	and r10d, 252
	xor r9d, r8d
	imul r11d, r9d, 668265261
	mov r9d, r11d
	shr r9d, 15
	xor r9d, r11d
	and r9d, 252
	xor eax, r8d
	imul eax, eax, 668265261
	mov r11d, eax
	shr r11d, 15
	xor r11d, eax
	and r11d, 252
	movaps xmm8, xmm0
	addps xmm8, xmm4
	movsd xmm9, qword ptr [rcx + 4*r11 + 4]
	mulps xmm9, xmm7
	xor edi, r8d
	imul eax, edi, 668265261
	mov edi, eax
	shr edi, 15
	xor edi, eax
	and edi, 252
	movsd xmm10, qword ptr [rcx + 4*rdi + 4]
	mulps xmm10, xmm7
	movaps xmm11, xmm3
	unpcklps xmm11, xmm5
	movss xmm12, dword ptr [rcx + 4*r11]
	movss xmm13, dword ptr [rcx + 4*rdi]
	unpcklps xmm13, xmm12
	mulps xmm13, xmm11
	movaps xmm11, xmm10
	unpcklps xmm11, xmm9
	addps xmm11, xmm13
	shufps xmm9, xmm10, 17
	shufps xmm9, xmm10, 226
	addps xmm9, xmm11
	movaps xmm10, xmm9
	shufps xmm10, xmm9, 85
	subss xmm9, xmm10
	mulss xmm9, xmm6
	addss xmm9, xmm10
	movss xmm8, xmm9
	shufps xmm5, xmm5, 0
	movss xmm9, dword ptr [rcx + 4*rsi]
	movss xmm10, dword ptr [rcx + 4*rsi + 4]
	movss xmm11, dword ptr [rcx + 4*rsi + 8]
	movss xmm12, dword ptr [rcx + 4*r10]
	unpcklps xmm12, xmm9
	mulps xmm12, xmm5
	movss xmm13, dword ptr [rcx + 4*r10 + 4]
	unpcklps xmm13, xmm10
	movss xmm10, dword ptr [rcx + 4*r10 + 8]
	movaps xmm9, xmm2
	shufps xmm9, xmm2, 0
	mulps xmm13, xmm9
	addps xmm13, xmm12
	shufps xmm11, xmm7, 212
	shufps xmm11, xmm7, 82
	movaps xmm5, xmm2
	movss xmm5, xmm10
	mulps xmm5, xmm11
	addps xmm5, xmm13
	shufps xmm3, xmm3, 0
	movss xmm10, dword ptr [rcx + 4*rdx]
	movss xmm11, dword ptr [rcx + 4*rdx + 4]
	movss xmm12, dword ptr [rcx + 4*rdx + 8]
	movss xmm13, dword ptr [rcx + 4*r9]
	unpcklps xmm13, xmm10
	mulps xmm13, xmm3
	movss xmm3, dword ptr [rcx + 4*r9 + 4]
	unpcklps xmm3, xmm11
	movss xmm10, dword ptr [rcx + 4*r9 + 8]
	unpcklps xmm10, xmm12
	mulps xmm3, xmm9
	addps xmm3, xmm13
	shufps xmm2, xmm7, 17
	shufps xmm2, xmm7, 226
	mulps xmm2, xmm10
	addps xmm2, xmm3
	subps xmm2, xmm5
	mulps xmm2, xmm6
	addps xmm2, xmm5
	subps xmm8, xmm2
	mulss xmm0, xmm4
	shufps xmm0, xmm0, 0
	mulps xmm0, xmm8
	addps xmm0, xmm2
	movaps xmm2, xmm0
	shufps xmm2, xmm0, 85
	subss xmm0, xmm2
	mulss xmm0, xmm1
	addss xmm0, xmm2
	mulss xmm0, dword ptr [rip + .LCPI22_6]
	pop rbx
	pop r14
	pop rbp
	ret