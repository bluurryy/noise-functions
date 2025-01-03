inspect_asm::open_simplex2_3d:
	push rbx
	movss xmm0, dword ptr [rdi]
	movsd xmm1, qword ptr [rdi + 4]
	movaps xmm2, xmm0
	addss xmm2, xmm1
	movaps xmm4, xmm1
	shufps xmm4, xmm1, 85
	addss xmm4, xmm2
	mulss xmm4, dword ptr [rip + .LCPI14_0]
	movaps xmm5, xmm4
	subss xmm5, xmm0
	shufps xmm4, xmm4, 0
	xorps xmm2, xmm2
	cmpnless xmm2, xmm5
	movss xmm0, dword ptr [rip + .LCPI14_1]
	andps xmm0, xmm2
	movss xmm3, dword ptr [rip + .LCPI14_2]
	andnps xmm2, xmm3
	orps xmm2, xmm0
	addss xmm2, xmm5
	cvttss2si edx, xmm2
	movss xmm0, dword ptr [rip + .LCPI14_3]
	ucomiss xmm2, xmm0
	mov edi, 2147483647
	cmova edx, edi
	xor esi, esi
	ucomiss xmm2, xmm2
	cmovp edx, esi
	subps xmm4, xmm1
	xorps xmm1, xmm1
	cmpnleps xmm1, xmm4
	movaps xmm2, xmm1
	andnps xmm2, xmmword ptr [rip + .LCPI14_4]
	andps xmm1, xmmword ptr [rip + .LCPI14_5]
	orps xmm1, xmm2
	addps xmm1, xmm4
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 255
	cvttss2si eax, xmm2
	ucomiss xmm2, xmm0
	cmova eax, edi
	ucomiss xmm2, xmm2
	cmovp eax, esi
	movaps xmm2, xmm1
	unpckhpd xmm2, xmm1
	cvttss2si ecx, xmm2
	ucomiss xmm2, xmm0
	cmova ecx, edi
	ucomiss xmm2, xmm2
	cmovp ecx, esi
	cvttss2si r9d, xmm1
	ucomiss xmm1, xmm0
	cmova r9d, edi
	movd xmm3, eax
	movd xmm2, ecx
	ucomiss xmm1, xmm1
	punpckldq xmm2, xmm3
	cmovp r9d, esi
	shufps xmm1, xmm1, 85
	cvttss2si r10d, xmm1
	ucomiss xmm1, xmm0
	movd xmm3, r9d
	cmova r10d, edi
	ucomiss xmm1, xmm1
	cmovp r10d, esi
	movd xmm1, r10d
	cvtsi2ss xmm6, edx
	punpckldq xmm3, xmm1
	subss xmm5, xmm6
	movss xmm1, dword ptr [rip + .LCPI14_6]
	subss xmm1, xmm5
	cvttss2si eax, xmm1
	ucomiss xmm1, xmm0
	punpcklqdq xmm3, xmm2
	cmova eax, edi
	cvtdq2ps xmm2, xmm3
	subps xmm4, xmm2
	ucomiss xmm1, xmm1
	cmovp eax, esi
	mov ecx, eax
	or ecx, 1
	movaps xmm1, xmmword ptr [rip + .LCPI14_7]
	subps xmm1, xmm4
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 255
	cvttss2si r8d, xmm2
	ucomiss xmm2, xmm0
	cmova r8d, edi
	ucomiss xmm2, xmm2
	cmovp r8d, esi
	movaps xmm2, xmm1
	unpckhpd xmm2, xmm1
	cvttss2si r11d, xmm2
	ucomiss xmm2, xmm0
	cmova r11d, edi
	ucomiss xmm2, xmm2
	cmovp r11d, esi
	cvttss2si ebx, xmm1
	ucomiss xmm1, xmm0
	cmova ebx, edi
	movd xmm2, r8d
	ucomiss xmm1, xmm1
	movd xmm3, r11d
	cmovp ebx, esi
	shufps xmm1, xmm1, 85
	cvttss2si r8d, xmm1
	ucomiss xmm1, xmm0
	cmova r8d, edi
	xorps xmm0, xmm0
	punpckldq xmm3, xmm2
	movd xmm6, ebx
	ucomiss xmm1, xmm1
	cmovp r8d, esi
	movd xmm1, r8d
	punpckldq xmm6, xmm1
	punpcklqdq xmm6, xmm3
	xorps xmm1, xmm1
	cvtsi2ss xmm1, ecx
	movdqa xmm7, xmmword ptr [rip + .LCPI14_8]
	por xmm7, xmm6
	movaps xmm8, xmmword ptr [rip + .LCPI14_9]
	movaps xmm9, xmm5
	xorps xmm9, xmm8
	mulss xmm9, xmm1
	cvtdq2ps xmm2, xmm7
	xorps xmm8, xmm4
	mulps xmm8, xmm2
	imul r8d, edx, 501125321
	imul edi, r9d, 1136930381
	imul esi, r10d, 1720413743
	movaps xmm10, xmm5
	mulss xmm10, xmm5
	movss xmm3, dword ptr [rip + .LCPI14_10]
	subss xmm3, xmm10
	movaps xmm10, xmm4
	mulps xmm10, xmm4
	movaps xmm11, xmm10
	shufps xmm11, xmm10, 85
	addss xmm11, xmm10
	subss xmm3, xmm11
	ucomiss xmm3, xmm0
	jbe .LBB_2
	movaps xmm0, xmm3
	mulss xmm0, xmm3
	mulss xmm0, xmm0
	mov edx, edi
	xor edx, esi
	xor edx, r8d
	imul edx, edx, 668265261
	mov r9d, edx
	shr r9d, 15
	xor r9d, edx
	and r9d, 252
	lea rdx, [rip + .L__unnamed__0]
	movss xmm10, dword ptr [rdx + 4*r9]
	mulss xmm10, xmm5
	movsd xmm11, qword ptr [rdx + 4*r9 + 4]
	mulps xmm11, xmm4
	addss xmm10, xmm11
	shufps xmm11, xmm11, 85
	addss xmm11, xmm10
	mulss xmm11, xmm0
	xorps xmm0, xmm0
	addss xmm0, xmm11
.LBB_2:
	movss xmm10, dword ptr [rip + .LCPI14_11]
	addss xmm10, xmm3
	ucomiss xmm9, xmm8
	movaps xmm11, xmm8
	shufps xmm11, xmm8, 85
	movd edx, xmm7
	pshufd xmm7, xmm7, 85
	jb .LBB_4
	ucomiss xmm9, xmm11
	jb .LBB_4
	addss xmm5, xmm1
	movaps xmm12, xmm1
	addss xmm12, xmm1
	mulss xmm12, xmm5
	imul r10d, ecx, -501125321
	add r10d, r8d
	movaps xmm11, xmm4
	shufps xmm11, xmm4, 85
	mov r11d, esi
	mov r9d, edi
	subss xmm10, xmm12
	xorps xmm12, xmm12
	ucomiss xmm10, xmm12
	ja .LBB_10
	jmp .LBB_11
.LBB_4:
	ucomiss xmm8, xmm9
	jbe .LBB_6
	ucomiss xmm8, xmm11
	jb .LBB_6
	movaps xmm13, xmm4
	addss xmm13, xmm2
	movaps xmm12, xmm2
	addss xmm12, xmm2
	mulss xmm12, xmm13
	imul r9d, edx, -1136930381
	add r9d, edi
	shufps xmm4, xmm4, 85
	movaps xmm11, xmm4
	movaps xmm4, xmm13
	mov r11d, esi
	jmp .LBB_8
.LBB_6:
	movaps xmm12, xmm2
	shufps xmm12, xmm2, 85
	movaps xmm11, xmm4
	addps xmm11, xmm2
	shufps xmm11, xmm11, 85
	addss xmm12, xmm12
	mulss xmm12, xmm11
	movd r9d, xmm7
	imul r11d, r9d, -1720413743
	add r11d, esi
	mov r9d, edi
.LBB_8:
	mov r10d, r8d
	subss xmm10, xmm12
	xorps xmm12, xmm12
	ucomiss xmm10, xmm12
	jbe .LBB_11
.LBB_10:
	mulss xmm10, xmm10
	mulss xmm10, xmm10
	xor r9d, r11d
	xor r9d, r10d
	imul r9d, r9d, 668265261
	mov r10d, r9d
	shr r10d, 15
	xor r10d, r9d
	and r10d, 252
	lea r9, [rip + .L__unnamed__0]
	mulss xmm5, dword ptr [r9 + 4*r10]
	mulss xmm4, dword ptr [r9 + 4*r10 + 4]
	mulss xmm11, dword ptr [r9 + 4*r10 + 8]
	addss xmm4, xmm5
	addss xmm11, xmm4
	mulss xmm11, xmm10
	addss xmm0, xmm11
.LBB_11:
	movss xmm5, dword ptr [rip + .LCPI14_2]
	subss xmm5, xmm9
	movaps xmm4, xmmword ptr [rip + .LCPI14_4]
	subps xmm4, xmm8
	mulss xmm1, xmm5
	mulps xmm2, xmm4
	movss xmm9, dword ptr [rip + .LCPI14_12]
	subss xmm9, xmm5
	movaps xmm8, xmm4
	shufps xmm8, xmm4, 85
	movaps xmm10, xmm4
	addss xmm10, xmm8
	subss xmm9, xmm10
	addss xmm3, xmm9
	shr eax
	and eax, 501125321
	add eax, r8d
	movd r8d, xmm6
	shr r8d
	and r8d, 1136930381
	add r8d, edi
	pshufd xmm6, xmm6, 85
	movd edi, xmm6
	shr edi
	and edi, 1720413743
	add edi, esi
	ucomiss xmm3, xmm12
	jbe .LBB_13
	movaps xmm6, xmm3
	mulss xmm6, xmm3
	mulss xmm6, xmm6
	mov esi, edi
	xor esi, r8d
	xor esi, eax
	not esi
	imul esi, esi, 668265261
	mov r9d, esi
	shr r9d, 15
	xor r9d, esi
	and r9d, 252
	lea rsi, [rip + .L__unnamed__0]
	movss xmm9, dword ptr [rsi + 4*r9]
	mulss xmm9, xmm1
	movsd xmm10, qword ptr [rsi + 4*r9 + 4]
	mulps xmm10, xmm2
	addss xmm9, xmm10
	shufps xmm10, xmm10, 85
	addss xmm10, xmm9
	mulss xmm10, xmm6
	addss xmm0, xmm10
.LBB_13:
	addss xmm3, dword ptr [rip + .LCPI14_11]
	ucomiss xmm5, xmm4
	jb .LBB_16
	ucomiss xmm5, xmm8
	jb .LBB_16
	imul edx, ecx, 501125321
	neg ecx
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	addss xmm1, xmm4
	addss xmm4, xmm4
	mulss xmm4, xmm1
	add eax, edx
	subss xmm3, xmm4
	xorps xmm4, xmm4
	ucomiss xmm3, xmm4
	ja .LBB_21
	jmp .LBB_22
.LBB_16:
	ucomiss xmm4, xmm5
	jbe .LBB_19
	ucomiss xmm4, xmm8
	jb .LBB_19
	imul ecx, edx, 1136930381
	neg edx
	xorps xmm4, xmm4
	cvtsi2ss xmm4, edx
	movaps xmm5, xmm2
	addss xmm5, xmm4
	addss xmm2, xmm4
	addss xmm4, xmm4
	mulss xmm4, xmm5
	add r8d, ecx
	subss xmm3, xmm4
	xorps xmm4, xmm4
	ucomiss xmm3, xmm4
	ja .LBB_21
	jmp .LBB_22
.LBB_19:
	movd ecx, xmm7
	imul edx, ecx, 1720413743
	neg ecx
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	movaps xmm5, xmm2
	shufps xmm5, xmm2, 85
	addss xmm5, xmm4
	addss xmm4, xmm4
	mulss xmm4, xmm5
	add edi, edx
	movlhps xmm5, xmm2
	shufps xmm5, xmm2, 226
	movaps xmm2, xmm5
	subss xmm3, xmm4
	xorps xmm4, xmm4
	ucomiss xmm3, xmm4
	jbe .LBB_22
.LBB_21:
	mulss xmm3, xmm3
	xor edi, r8d
	xor edi, eax
	not edi
	imul eax, edi, 668265261
	mov ecx, eax
	shr ecx, 15
	xor ecx, eax
	and ecx, 252
	lea rax, [rip + .L__unnamed__0]
	mulss xmm1, dword ptr [rax + 4*rcx]
	mulss xmm3, xmm3
	movsd xmm4, qword ptr [rax + 4*rcx + 4]
	mulps xmm2, xmm4
	addss xmm1, xmm2
	shufps xmm2, xmm2, 85
	addss xmm2, xmm1
	mulss xmm2, xmm3
	addss xmm0, xmm2
.LBB_22:
	mulss xmm0, dword ptr [rip + .LCPI14_13]
	pop rbx
	ret