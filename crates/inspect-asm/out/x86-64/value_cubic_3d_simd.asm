inspect_asm::value_cubic_3d_simd:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	sub rsp, 40
	movaps xmm1, xmmword ptr [rdi]
	cvttps2dq xmm0, xmm1
	xorps xmm10, xmm10
	cmpnleps xmm10, xmm1
	movaps xmm9, xmm1
	paddd xmm10, xmm0
	cvtdq2ps xmm4, xmm10
	pshufd xmm1, xmm10, 245
	pmuludq xmm10, xmmword ptr [rip + .LCPI28_0]
	pshufd xmm3, xmm10, 232
	pmuludq xmm1, xmmword ptr [rip + .LCPI28_1]
	pshufd xmm0, xmm1, 232
	punpckldq xmm3, xmm0
	movdqa xmm2, xmmword ptr [rip + .LCPI28_2]
	paddd xmm2, xmm3
	movdqa xmm0, xmmword ptr [rip + .LCPI28_3]
	paddd xmm0, xmm3
	movdqa xmm6, xmm0
	movdqa xmm0, xmm3
	movd r12d, xmm2
	pshufd xmm3, xmm2, 85
	movd r11d, xmm3
	pshufd xmm2, xmm2, 238
	movd eax, xmm2
	mov ecx, r12d
	xor ecx, r11d
	mov dword ptr [rsp - 112], ecx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm3, xmm3
	cvtsi2ss xmm3, edx
	movd r10d, xmm10
	mov r9d, r11d
	xor r9d, r10d
	mov ecx, r9d
	mov dword ptr [rsp], r9d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm5, edx
	movdqa xmm2, xmm0
	paddd xmm2, xmmword ptr [rip + .LCPI28_4]
	movdqa xmmword ptr [rsp - 48], xmm2
	movd r13d, xmm6
	movdqa xmm8, xmm6
	movdqa xmmword ptr [rsp - 32], xmm6
	mov r8d, r13d
	xor r8d, r11d
	mov ecx, r8d
	mov dword ptr [rsp - 16], r8d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm0, xmm0
	cvtsi2ss xmm0, edx
	movd r14d, xmm2
	xor r11d, r14d
	mov ecx, r11d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm6, xmm6
	cvtsi2ss xmm6, edx
	movd esi, xmm1
	mov edx, r12d
	xor edx, esi
	mov ecx, edx
	mov r15d, edx
	mov dword ptr [rsp - 88], edx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm12, edx
	mov ecx, r10d
	xor ecx, esi
	mov dword ptr [rsp - 100], ecx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm1, edx
	mov ecx, r13d
	xor ecx, esi
	mov dword ptr [rsp - 104], ecx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm11, edx
	mov ecx, esi
	xor ecx, r14d
	mov dword ptr [rsp - 108], ecx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm7, edx
	pshufd xmm2, xmm8, 85
	movd ebx, xmm2
	mov edi, r12d
	xor edi, ebx
	mov ecx, edi
	mov dword ptr [rsp - 96], edi
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	subps xmm9, xmm4
	movss xmm2, dword ptr [rip + .LCPI28_5]
	mulss xmm3, xmm2
	mulss xmm5, xmm2
	movaps xmm4, xmm0
	mulss xmm4, xmm2
	mulss xmm6, xmm2
	movaps xmm0, xmm2
	subss xmm6, xmm4
	subss xmm4, xmm3
	movaps xmm15, xmm4
	subss xmm3, xmm5
	subss xmm6, xmm3
	movaps xmm2, xmm9
	mulss xmm2, xmm9
	movaps xmm8, xmm9
	mulss xmm8, xmm2
	movaps xmm4, xmm8
	mulss xmm4, xmm6
	subss xmm3, xmm6
	cvtsi2ss xmm13, edx
	mov ecx, ebx
	xor ecx, r10d
	mov dword ptr [rsp - 116], ecx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm14, edx
	mulss xmm3, xmm2
	movaps xmm6, xmm2
	addss xmm3, xmm4
	mov edx, r13d
	xor edx, ebx
	mov ecx, edx
	mov dword ptr [rsp - 84], edx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm15, xmm9
	addss xmm15, xmm3
	xorps xmm2, xmm2
	cvtsi2ss xmm2, esi
	xor ebx, r14d
	mov ecx, ebx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm15, xmm5
	movss dword ptr [rsp - 52], xmm15
	mulss xmm12, xmm0
	mulss xmm1, xmm0
	mulss xmm11, xmm0
	mulss xmm7, xmm0
	subss xmm7, xmm11
	subss xmm11, xmm12
	movaps xmm15, xmm11
	subss xmm12, xmm1
	subss xmm7, xmm12
	movaps xmm4, xmm8
	mulss xmm4, xmm7
	subss xmm12, xmm7
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	pshufd xmm5, xmmword ptr [rsp - 48], 85
	movd ecx, xmm5
	xor r12d, ecx
	mov esi, r12d
	xor esi, eax
	imul esi, esi, 668265261
	imul esi, esi
	mov ebp, esi
	shl ebp, 19
	xor ebp, esi
	xorps xmm11, xmm11
	cvtsi2ss xmm11, ebp
	mulss xmm12, xmm6
	addss xmm12, xmm4
	xor r10d, ecx
	mov esi, r10d
	xor esi, eax
	imul esi, esi, 668265261
	imul esi, esi
	mov ebp, esi
	shl ebp, 19
	xor ebp, esi
	mulss xmm15, xmm9
	addss xmm15, xmm12
	xorps xmm12, xmm12
	cvtsi2ss xmm12, ebp
	xor r13d, ecx
	mov esi, r13d
	xor esi, eax
	imul esi, esi, 668265261
	imul esi, esi
	mov ebp, esi
	shl ebp, 19
	xor ebp, esi
	addss xmm15, xmm1
	movss dword ptr [rsp - 56], xmm15
	mulss xmm13, xmm0
	mulss xmm14, xmm0
	mulss xmm2, xmm0
	mulss xmm3, xmm0
	subss xmm3, xmm2
	subss xmm2, xmm13
	movaps xmm5, xmm2
	subss xmm13, xmm14
	subss xmm3, xmm13
	movaps xmm4, xmm8
	xorps xmm2, xmm2
	cvtsi2ss xmm2, ebp
	mulss xmm4, xmm3
	subss xmm13, xmm3
	xor r14d, ecx
	xor eax, r14d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm3, xmm3
	cvtsi2ss xmm3, ecx
	pshufd xmm1, xmm10, 238
	movd eax, xmm1
	mov ebp, dword ptr [rsp - 112]
	mov ecx, ebp
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	xorps xmm1, xmm1
	cvtsi2ss xmm1, esi
	mulss xmm13, xmm6
	addss xmm13, xmm4
	xor r9d, eax
	imul ecx, r9d, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	xorps xmm15, xmm15
	cvtsi2ss xmm15, esi
	mulss xmm5, xmm9
	addss xmm5, xmm13
	xor r8d, eax
	imul ecx, r8d, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm5, xmm14
	movss dword ptr [rsp - 124], xmm5
	mulss xmm11, xmm0
	mulss xmm12, xmm0
	mulss xmm2, xmm0
	mulss xmm3, xmm0
	subss xmm3, xmm2
	subss xmm2, xmm11
	subss xmm11, xmm12
	subss xmm3, xmm11
	movaps xmm4, xmm8
	mulss xmm4, xmm3
	subss xmm11, xmm3
	xorps xmm5, xmm5
	cvtsi2ss xmm5, esi
	mov ecx, r11d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm11, xmm6
	addss xmm11, xmm4
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	xor edi, eax
	imul ecx, edi, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	movaps xmm13, xmm9
	mulss xmm2, xmm9
	addss xmm2, xmm11
	xorps xmm4, xmm4
	cvtsi2ss xmm4, esi
	mov r9d, dword ptr [rsp - 116]
	mov ecx, r9d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm2, xmm12
	movss dword ptr [rsp - 60], xmm2
	mulss xmm1, xmm0
	mulss xmm15, xmm0
	mulss xmm5, xmm0
	mulss xmm3, xmm0
	subss xmm3, xmm5
	subss xmm5, xmm1
	movaps xmm2, xmm5
	subss xmm1, xmm15
	subss xmm3, xmm1
	movaps xmm5, xmm8
	cvtsi2ss xmm7, esi
	mulss xmm5, xmm3
	subss xmm1, xmm3
	mov ecx, edx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	xorps xmm11, xmm11
	cvtsi2ss xmm11, esi
	mulss xmm1, xmm6
	addss xmm1, xmm5
	mov ecx, ebx
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	mulss xmm2, xmm9
	addss xmm2, xmm1
	mov ecx, r12d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm2, xmm15
	movss dword ptr [rsp - 92], xmm2
	mulss xmm4, xmm0
	mulss xmm7, xmm0
	mulss xmm11, xmm0
	mulss xmm3, xmm0
	movaps xmm2, xmm0
	subss xmm3, xmm11
	subss xmm11, xmm4
	subss xmm4, xmm7
	subss xmm3, xmm4
	movaps xmm5, xmm8
	mulss xmm5, xmm3
	subss xmm4, xmm3
	xorps xmm1, xmm1
	cvtsi2ss xmm1, esi
	mov ecx, r10d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm4, xmm6
	addss xmm4, xmm5
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	mov ecx, r13d
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm11, xmm9
	addss xmm11, xmm4
	xorps xmm5, xmm5
	cvtsi2ss xmm5, esi
	mov ecx, r15d
	xor ecx, eax
	mov dword ptr [rsp - 76], ecx
	mov r15d, dword ptr [rsp - 100]
	mov ecx, r15d
	xor ecx, eax
	mov dword ptr [rsp - 72], ecx
	mov edi, dword ptr [rsp - 104]
	mov ecx, edi
	xor ecx, eax
	mov dword ptr [rsp - 68], ecx
	mov r8d, dword ptr [rsp - 108]
	mov ecx, r8d
	xor ecx, eax
	mov dword ptr [rsp - 64], ecx
	xor eax, r14d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	pshufd xmm0, xmmword ptr [rsp - 32], 238
	movd edx, xmm0
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	mov ecx, ebp
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm11, xmm7
	mulss xmm1, xmm2
	mulss xmm3, xmm2
	mulss xmm5, xmm2
	mulss xmm4, xmm2
	movaps xmm7, xmm2
	subss xmm4, xmm5
	subss xmm5, xmm1
	movaps xmm9, xmm5
	subss xmm1, xmm3
	subss xmm4, xmm1
	movaps xmm5, xmm8
	mulss xmm5, xmm4
	xorps xmm0, xmm0
	cvtsi2ss xmm0, esi
	subss xmm1, xmm4
	mov ecx, dword ptr [rsp]
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm1, xmm6
	cvtsi2ss xmm2, esi
	addss xmm1, xmm5
	mov ecx, dword ptr [rsp - 16]
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm9, xmm13
	cvtsi2ss xmm10, esi
	addss xmm9, xmm1
	mov ecx, r11d
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm9, xmm3
	movss dword ptr [rsp - 32], xmm9
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	mulss xmm0, xmm7
	mulss xmm2, xmm7
	mulss xmm10, xmm7
	mulss xmm3, xmm7
	movaps xmm9, xmm7
	subss xmm3, xmm10
	subss xmm10, xmm0
	subss xmm0, xmm2
	subss xmm3, xmm0
	movaps xmm1, xmm8
	mulss xmm1, xmm3
	subss xmm0, xmm3
	mov ecx, dword ptr [rsp - 96]
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	movaps xmm7, xmm6
	mulss xmm0, xmm6
	addss xmm0, xmm1
	xorps xmm4, xmm4
	cvtsi2ss xmm4, esi
	xor r9d, edx
	imul ecx, r9d, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mov r9d, dword ptr [rsp - 84]
	mov ecx, r9d
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov ebp, ecx
	shl ebp, 19
	xor ebp, ecx
	cvtsi2ss xmm5, esi
	mulss xmm10, xmm13
	cvtsi2ss xmm14, ebp
	addss xmm10, xmm0
	addss xmm10, xmm2
	movss dword ptr [rsp - 120], xmm10
	mov ecx, ebx
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm4, xmm9
	mulss xmm5, xmm9
	mulss xmm14, xmm9
	xorps xmm0, xmm0
	cvtsi2ss xmm0, esi
	mulss xmm0, xmm9
	subss xmm0, xmm14
	subss xmm14, xmm4
	subss xmm4, xmm5
	subss xmm0, xmm4
	movaps xmm3, xmm8
	mulss xmm3, xmm0
	subss xmm4, xmm0
	mulss xmm4, xmm6
	addss xmm4, xmm3
	mov ecx, r12d
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mulss xmm14, xmm13
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	addss xmm14, xmm4
	mov ecx, r10d
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	addss xmm14, xmm5
	xorps xmm4, xmm4
	cvtsi2ss xmm4, esi
	mov ecx, r13d
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	mov ecx, dword ptr [rsp - 88]
	xor ecx, edx
	mov ebp, r15d
	xor ebp, edx
	mov r15d, edi
	xor r15d, edx
	xor r8d, edx
	mov dword ptr [rsp - 80], r8d
	xor edx, r14d
	imul eax, edx, 668265261
	imul eax, eax
	mov edx, eax
	shl edx, 19
	xor edx, eax
	xorps xmm10, xmm10
	cvtsi2ss xmm10, esi
	mulss xmm3, xmm9
	xorps xmm0, xmm0
	cvtsi2ss xmm0, edx
	mulss xmm4, xmm9
	mulss xmm10, xmm9
	mulss xmm0, xmm9
	subss xmm0, xmm10
	subss xmm10, xmm3
	subss xmm3, xmm4
	subss xmm0, xmm3
	movaps xmm5, xmm8
	mulss xmm5, xmm0
	subss xmm3, xmm0
	mulss xmm3, xmm6
	addss xmm3, xmm5
	pshufd xmm0, xmmword ptr [rsp - 48], 238
	movd eax, xmm0
	mov edx, dword ptr [rsp - 112]
	xor edx, eax
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	mulss xmm10, xmm13
	xorps xmm0, xmm0
	cvtsi2ss xmm0, esi
	addss xmm10, xmm3
	mov edx, dword ptr [rsp]
	xor edx, eax
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	addss xmm10, xmm4
	xorps xmm3, xmm3
	cvtsi2ss xmm3, esi
	mov edx, dword ptr [rsp - 16]
	xor edx, eax
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	xor r11d, eax
	imul edx, r11d, 668265261
	imul edx, edx
	mov r11d, edx
	shl r11d, 19
	xor r11d, edx
	movaps xmm1, xmm9
	mulss xmm0, xmm9
	mulss xmm3, xmm9
	cvtsi2ss xmm9, esi
	mulss xmm9, xmm1
	xorps xmm4, xmm4
	cvtsi2ss xmm4, r11d
	mulss xmm4, xmm1
	subss xmm4, xmm9
	subss xmm9, xmm0
	subss xmm0, xmm3
	subss xmm4, xmm0
	movaps xmm5, xmm8
	mulss xmm5, xmm4
	subss xmm0, xmm4
	mulss xmm0, xmm6
	addss xmm0, xmm5
	mov edx, dword ptr [rsp - 96]
	xor edx, eax
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	mulss xmm9, xmm13
	xorps xmm4, xmm4
	cvtsi2ss xmm4, esi
	addss xmm9, xmm0
	addss xmm9, xmm3
	mov edx, dword ptr [rsp - 116]
	xor edx, eax
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	xor r9d, eax
	imul edx, r9d, 668265261
	imul edx, edx
	mov r11d, edx
	shl r11d, 19
	xor r11d, edx
	xor ebx, eax
	imul edx, ebx, 668265261
	imul edx, edx
	mov ebx, edx
	shl ebx, 19
	xor ebx, edx
	mulss xmm4, xmm1
	xorps xmm0, xmm0
	cvtsi2ss xmm0, esi
	mulss xmm0, xmm1
	cvtsi2ss xmm15, r11d
	mulss xmm15, xmm1
	xorps xmm3, xmm3
	cvtsi2ss xmm3, ebx
	mulss xmm3, xmm1
	subss xmm3, xmm15
	subss xmm15, xmm4
	subss xmm4, xmm0
	subss xmm3, xmm4
	movaps xmm5, xmm8
	mulss xmm5, xmm3
	subss xmm4, xmm3
	movaps xmmword ptr [rsp + 16], xmm6
	mulss xmm4, xmm6
	addss xmm4, xmm5
	mulss xmm15, xmm13
	addss xmm15, xmm4
	addss xmm15, xmm0
	xor r12d, eax
	imul esi, r12d, 668265261
	imul esi, esi
	mov edx, esi
	shl edx, 19
	xor edx, esi
	xor r10d, eax
	imul esi, r10d, 668265261
	imul esi, esi
	mov edi, esi
	shl edi, 19
	xor edi, esi
	xor r13d, eax
	imul esi, r13d, 668265261
	imul esi, esi
	mov r8d, esi
	shl r8d, 19
	xor r8d, esi
	xor r14d, eax
	imul esi, r14d, 668265261
	imul esi, esi
	mov r9d, esi
	shl r9d, 19
	xor r9d, esi
	xorps xmm3, xmm3
	cvtsi2ss xmm3, edx
	movaps xmm2, xmm1
	mulss xmm3, xmm1
	cvtsi2ss xmm1, edi
	mulss xmm1, xmm2
	cvtsi2ss xmm6, r8d
	mulss xmm6, xmm2
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r9d
	mulss xmm0, xmm2
	movaps xmm5, xmm2
	subss xmm0, xmm6
	subss xmm6, xmm3
	subss xmm3, xmm1
	subss xmm0, xmm3
	movaps xmm12, xmm8
	movaps xmmword ptr [rsp - 16], xmm8
	movaps xmmword ptr [rsp], xmm8
	mulss xmm8, xmm0
	subss xmm3, xmm0
	mulss xmm3, xmm7
	addss xmm3, xmm8
	mulss xmm6, xmm13
	addss xmm6, xmm3
	addss xmm6, xmm1
	movaps xmm3, xmm13
	shufps xmm3, xmm13, 85
	movss xmm4, dword ptr [rsp - 124]
	movss xmm7, dword ptr [rsp - 60]
	subss xmm7, xmm4
	movss xmm0, dword ptr [rsp - 52]
	subss xmm4, xmm0
	movss xmm2, dword ptr [rsp - 56]
	subss xmm0, xmm2
	subss xmm7, xmm0
	mulss xmm4, xmm3
	movss xmm1, dword ptr [rsp - 32]
	subss xmm1, xmm11
	movss dword ptr [rsp - 32], xmm1
	subss xmm11, dword ptr [rsp - 92]
	mulss xmm11, xmm3
	movss dword ptr [rsp - 48], xmm11
	subss xmm10, xmm14
	subss xmm14, dword ptr [rsp - 120]
	mulss xmm14, xmm3
	subss xmm6, xmm15
	subss xmm15, xmm9
	mulss xmm15, xmm3
	movaps xmm8, xmm3
	movaps xmm11, xmm3
	mulss xmm11, xmm3
	mulss xmm8, xmm11
	movaps xmm1, xmm8
	mulss xmm1, xmm7
	subss xmm0, xmm7
	mulss xmm0, xmm11
	addss xmm0, xmm1
	addss xmm4, xmm0
	addss xmm4, xmm2
	movss dword ptr [rsp - 124], xmm4
	imul edx, dword ptr [rsp - 76], 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	imul edx, dword ptr [rsp - 72], 668265261
	imul edx, edx
	mov edi, edx
	shl edi, 19
	xor edi, edx
	imul edx, dword ptr [rsp - 68], 668265261
	imul edx, edx
	mov r8d, edx
	shl r8d, 19
	xor r8d, edx
	imul edx, dword ptr [rsp - 64], 668265261
	imul edx, edx
	mov r9d, edx
	shl r9d, 19
	xorps xmm1, xmm1
	cvtsi2ss xmm1, esi
	xor r9d, edx
	xorps xmm3, xmm3
	cvtsi2ss xmm3, edi
	movaps xmm7, xmm5
	mulss xmm1, xmm5
	xorps xmm2, xmm2
	cvtsi2ss xmm2, r8d
	mulss xmm3, xmm5
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r9d
	mulss xmm2, xmm5
	mulss xmm0, xmm5
	subss xmm0, xmm2
	subss xmm2, xmm1
	subss xmm1, xmm3
	subss xmm0, xmm1
	mulss xmm12, xmm0
	subss xmm1, xmm0
	movaps xmm5, xmmword ptr [rsp + 16]
	mulss xmm1, xmm5
	addss xmm1, xmm12
	mulss xmm2, xmm13
	addss xmm2, xmm1
	addss xmm2, xmm3
	movss xmm1, dword ptr [rsp - 92]
	subss xmm1, xmm2
	movss xmm3, dword ptr [rsp - 32]
	subss xmm3, xmm1
	movaps xmm0, xmm8
	mulss xmm0, xmm3
	subss xmm1, xmm3
	mulss xmm1, xmm11
	addss xmm1, xmm0
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm12, xmm12
	cvtsi2ss xmm12, edx
	movss xmm0, dword ptr [rsp - 48]
	addss xmm0, xmm1
	addss xmm0, xmm2
	movss dword ptr [rsp - 48], xmm0
	imul ecx, ebp, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	imul ecx, r15d, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	imul ecx, dword ptr [rsp - 80], 668265261
	imul ecx, ecx
	mov edi, ecx
	shl edi, 19
	xor edi, ecx
	xorps xmm0, xmm0
	cvtsi2ss xmm0, edx
	mulss xmm12, xmm7
	cvtsi2ss xmm4, esi
	mulss xmm0, xmm7
	xorps xmm1, xmm1
	cvtsi2ss xmm1, edi
	mulss xmm4, xmm7
	mulss xmm1, xmm7
	subss xmm1, xmm4
	subss xmm4, xmm12
	subss xmm12, xmm0
	subss xmm1, xmm12
	movaps xmm2, xmmword ptr [rsp - 16]
	mulss xmm2, xmm1
	subss xmm12, xmm1
	movaps xmm3, xmm5
	mulss xmm12, xmm5
	addss xmm12, xmm2
	mulss xmm4, xmm13
	addss xmm4, xmm12
	mov ecx, dword ptr [rsp - 88]
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	addss xmm4, xmm0
	movss xmm1, dword ptr [rsp - 120]
	subss xmm1, xmm4
	subss xmm10, xmm1
	movaps xmm0, xmm8
	mulss xmm0, xmm10
	subss xmm1, xmm10
	cvtsi2ss xmm5, edx
	mov ecx, dword ptr [rsp - 100]
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm10, xmm10
	cvtsi2ss xmm10, edx
	mulss xmm1, xmm11
	addss xmm1, xmm0
	movaps xmm2, xmm1
	mov ecx, dword ptr [rsp - 104]
	xor ecx, eax
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov ecx, dword ptr [rsp - 108]
	xor ecx, eax
	imul eax, ecx, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xorps xmm0, xmm0
	cvtsi2ss xmm0, edx
	xor ecx, eax
	xorps xmm1, xmm1
	cvtsi2ss xmm1, ecx
	addss xmm14, xmm2
	addss xmm14, xmm4
	mulss xmm5, xmm7
	mulss xmm10, xmm7
	mulss xmm0, xmm7
	mulss xmm1, xmm7
	subss xmm1, xmm0
	subss xmm0, xmm5
	subss xmm5, xmm10
	subss xmm1, xmm5
	movaps xmm2, xmmword ptr [rsp]
	mulss xmm2, xmm1
	subss xmm5, xmm1
	mulss xmm5, xmm3
	addss xmm5, xmm2
	mulss xmm0, xmm13
	addss xmm0, xmm5
	addss xmm0, xmm10
	subss xmm9, xmm0
	subss xmm6, xmm9
	mulss xmm8, xmm6
	subss xmm9, xmm6
	mulss xmm9, xmm11
	addss xmm9, xmm8
	addss xmm15, xmm9
	addss xmm15, xmm0
	movhlps xmm13, xmm13
	subss xmm15, xmm14
	movss xmm0, dword ptr [rsp - 124]
	subss xmm14, xmm0
	movss xmm3, dword ptr [rsp - 48]
	subss xmm0, xmm3
	subss xmm15, xmm0
	movaps xmm2, xmm0
	movaps xmm0, xmm13
	mulss xmm14, xmm13
	mulss xmm13, xmm13
	mulss xmm0, xmm13
	mulss xmm0, xmm15
	subss xmm2, xmm15
	mulss xmm2, xmm13
	addss xmm2, xmm0
	addss xmm14, xmm2
	addss xmm14, xmm3
	mulss xmm14, dword ptr [rip + .LCPI28_6]
	movaps xmm0, xmm14
	add rsp, 40
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret