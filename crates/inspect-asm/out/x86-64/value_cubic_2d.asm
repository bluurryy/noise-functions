inspect_asm::value_cubic_2d:
	push rbx
	movd xmm2, edi
	mov rax, rdi
	shr rax, 32
	movd xmm0, eax
	punpckldq xmm0, xmm2
	cvttss2si esi, xmm0
	movss xmm3, dword ptr [rip + .LCPI25_0]
	ucomiss xmm0, xmm3
	mov ecx, 2147483647
	cmova esi, ecx
	xor edx, edx
	ucomiss xmm0, xmm0
	cmovp esi, edx
	cvttss2si r8d, xmm2
	ucomiss xmm2, xmm3
	movd xmm6, edi
	cmova r8d, ecx
	xorps xmm4, xmm4
	movd xmm5, esi
	ucomiss xmm2, xmm2
	cmovp r8d, edx
	ucomiss xmm0, xmm3
	movd xmm2, r8d
	cmova ecx, ecx
	ucomiss xmm0, xmm0
	cmovp ecx, edx
	punpckldq xmm5, xmm2
	movd xmm2, ecx
	pshufd xmm2, xmm2, 0
	punpcklqdq xmm5, xmm2
	cmpnleps xmm4, xmm0
	paddd xmm4, xmm5
	pshufd xmm0, xmm4, 85
	cvtdq2ps xmm0, xmm0
	subss xmm6, xmm0
	cvtdq2ps xmm0, xmm4
	movd xmm1, eax
	pshufd xmm2, xmm4, 245
	pmuludq xmm4, xmmword ptr [rip + .LCPI25_1]
	pmuludq xmm2, xmmword ptr [rip + .LCPI25_2]
	subss xmm1, xmm0
	movss dword ptr [rsp - 4], xmm1
	movd eax, xmm2
	lea esi, [rax - 501125321]
	movd r10d, xmm4
	lea r8d, [r10 - 1136930381]
	lea ecx, [rax + 501125321]
	mov edx, r8d
	xor edx, esi
	imul edx, edx, 668265261
	imul edx, edx
	mov edi, edx
	shl edi, 19
	xor edi, edx
	xorps xmm5, xmm5
	cvtsi2ss xmm5, edi
	lea r9d, [r10 + 1136930381]
	lea edx, [rax + 1002250642]
	mov edi, r8d
	xor edi, eax
	imul edi, edi, 668265261
	imul edi, edi
	mov r11d, edi
	shl r11d, 19
	xor r11d, edi
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r11d
	lea edi, [r10 - 2021106534]
	movss xmm4, dword ptr [rip + .LCPI25_3]
	mov r11d, r8d
	xor r11d, ecx
	imul r11d, r11d, 668265261
	imul r11d, r11d
	mov ebx, r11d
	shl ebx, 19
	xor ebx, r11d
	xorps xmm2, xmm2
	cvtsi2ss xmm2, ebx
	mulss xmm5, xmm4
	mulss xmm0, xmm4
	movss dword ptr [rsp - 8], xmm0
	mulss xmm2, xmm4
	xor r8d, edx
	imul r8d, r8d, 668265261
	imul r8d, r8d
	mov r11d, r8d
	shl r11d, 19
	xor r11d, r8d
	cvtsi2ss xmm8, r11d
	mulss xmm8, xmm4
	subss xmm8, xmm2
	subss xmm2, xmm5
	subss xmm5, xmm0
	subss xmm8, xmm5
	mulss xmm2, xmm6
	mov r8d, r10d
	xor r8d, esi
	imul r8d, r8d, 668265261
	imul r8d, r8d
	mov r11d, r8d
	shl r11d, 19
	xor r11d, r8d
	cvtsi2ss xmm9, r11d
	mulss xmm9, xmm4
	mov r8d, r10d
	mov r11d, r10d
	xor r11d, ecx
	imul r11d, r11d, 668265261
	imul r11d, r11d
	mov ebx, r11d
	shl ebx, 19
	xor ebx, r11d
	cvtsi2ss xmm7, ebx
	xor r8d, eax
	mulss xmm7, xmm4
	xor r10d, edx
	imul r10d, r10d, 668265261
	imul r10d, r10d
	mov r11d, r10d
	shl r11d, 19
	xor r11d, r10d
	cvtsi2ss xmm10, r11d
	mulss xmm10, xmm4
	subss xmm10, xmm7
	subss xmm7, xmm9
	mov r10d, r9d
	xor r10d, esi
	imul r10d, r10d, 668265261
	imul r10d, r10d
	mov r11d, r10d
	shl r11d, 19
	xor r11d, r10d
	cvtsi2ss xmm11, r11d
	mulss xmm7, xmm6
	mulss xmm11, xmm4
	mov r10d, r9d
	xor r10d, ecx
	imul r10d, r10d, 668265261
	imul r10d, r10d
	mov r11d, r10d
	shl r11d, 19
	xor r11d, r10d
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r11d
	mov r10d, r9d
	xor r10d, eax
	mulss xmm0, xmm4
	xor r9d, edx
	imul r9d, r9d, 668265261
	imul r9d, r9d
	mov r11d, r9d
	shl r11d, 19
	xor r11d, r9d
	cvtsi2ss xmm12, r11d
	mulss xmm12, xmm4
	subss xmm12, xmm0
	subss xmm0, xmm11
	xor esi, edi
	imul esi, esi, 668265261
	imul esi, esi
	mov r9d, esi
	shl r9d, 19
	xor r9d, esi
	cvtsi2ss xmm14, r9d
	mulss xmm0, xmm6
	xor eax, edi
	xor ecx, edi
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov esi, ecx
	shl esi, 19
	xor esi, ecx
	cvtsi2ss xmm13, esi
	xor edi, edx
	imul ecx, edi, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm15, edx
	mulss xmm14, xmm4
	mulss xmm13, xmm4
	mulss xmm15, xmm4
	subss xmm15, xmm13
	subss xmm13, xmm14
	mulss xmm13, xmm6
	movaps xmm1, xmm6
	mulss xmm6, xmm6
	mulss xmm1, xmm6
	movaps xmm3, xmm1
	mulss xmm3, xmm8
	subss xmm5, xmm8
	mulss xmm5, xmm6
	addss xmm5, xmm3
	addss xmm2, xmm5
	imul ecx, r8d, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm3, xmm3
	cvtsi2ss xmm3, edx
	addss xmm2, dword ptr [rsp - 8]
	mulss xmm3, xmm4
	subss xmm9, xmm3
	subss xmm10, xmm9
	movaps xmm5, xmm1
	mulss xmm5, xmm10
	subss xmm9, xmm10
	mulss xmm9, xmm6
	addss xmm9, xmm5
	addss xmm7, xmm9
	addss xmm7, xmm3
	imul ecx, r10d, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm3, xmm3
	cvtsi2ss xmm3, edx
	mulss xmm3, xmm4
	subss xmm11, xmm3
	subss xmm12, xmm11
	movaps xmm5, xmm1
	mulss xmm5, xmm12
	subss xmm11, xmm12
	mulss xmm11, xmm6
	addss xmm11, xmm5
	addss xmm0, xmm11
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm5, xmm5
	cvtsi2ss xmm5, ecx
	addss xmm0, xmm3
	mulss xmm5, xmm4
	subss xmm14, xmm5
	subss xmm15, xmm14
	mulss xmm1, xmm15
	subss xmm14, xmm15
	mulss xmm14, xmm6
	addss xmm14, xmm1
	addss xmm13, xmm14
	addss xmm13, xmm5
	subss xmm13, xmm0
	subss xmm0, xmm2
	subss xmm2, xmm7
	subss xmm13, xmm2
	movss xmm3, dword ptr [rsp - 4]
	movaps xmm1, xmm3
	mulss xmm0, xmm3
	mulss xmm3, xmm3
	mulss xmm1, xmm3
	mulss xmm1, xmm13
	subss xmm2, xmm13
	mulss xmm2, xmm3
	addss xmm2, xmm1
	addss xmm0, xmm2
	addss xmm0, xmm7
	mulss xmm0, dword ptr [rip + .LCPI25_4]
	pop rbx
	ret