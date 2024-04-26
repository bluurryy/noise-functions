inspect_asm::value_cubic_3d:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	sub rsp, 168
	movss xmm2, dword ptr [rdi + 8]
	cvttss2si eax, xmm2
	movss xmm0, dword ptr [rip + .LCPI26_0]
	ucomiss xmm2, xmm0
	mov ecx, 2147483647
	cmova eax, ecx
	xor edx, edx
	ucomiss xmm2, xmm2
	cmovp eax, edx
	xorps xmm1, xmm1
	ucomiss xmm2, xmm1
	movaps xmm5, xmm2
	sbb eax, 0
	cvtsi2ss xmm10, eax
	ucomiss xmm0, xmm0
	cmova esi, ecx
	movsd xmm1, qword ptr [rdi]
	ucomiss xmm0, xmm0
	cmovp esi, edx
	movaps xmm2, xmm1
	shufps xmm2, xmm1, 255
	cvttss2si edi, xmm2
	ucomiss xmm2, xmm0
	xorps xmm3, xmm3
	cmova edi, ecx
	ucomiss xmm2, xmm2
	cmovp edi, edx
	cvttss2si r8d, xmm1
	ucomiss xmm1, xmm0
	movd xmm2, esi
	cmova r8d, ecx
	movd xmm4, edi
	punpckldq xmm2, xmm4
	ucomiss xmm1, xmm1
	cmovp r8d, edx
	movaps xmm4, xmm1
	shufps xmm4, xmm1, 85
	cvttss2si esi, xmm4
	ucomiss xmm4, xmm0
	movd xmm0, r8d
	cmova esi, ecx
	ucomiss xmm4, xmm4
	movaps xmm6, xmm4
	cmovp esi, edx
	movd xmm4, esi
	punpckldq xmm0, xmm4
	punpcklqdq xmm0, xmm2
	cmpnleps xmm3, xmm1
	paddd xmm3, xmm0
	cvtdq2ps xmm2, xmm3
	pshufd xmm0, xmm3, 85
	pshufd xmm4, xmm3, 245
	pmuludq xmm4, xmmword ptr [rip + .LCPI26_1]
	pmuludq xmm3, xmmword ptr [rip + .LCPI26_2]
	imul r15d, eax, 1720413743
	movd r10d, xmm3
	lea ebp, [r10 - 501125321]
	movd ebx, xmm4
	lea r13d, [rbx - 1136930381]
	lea eax, [r15 - 1720413743]
	lea r12d, [r10 + 501125321]
	mov edx, r13d
	xor edx, ebp
	mov dword ptr [rsp - 116], edx
	mov ecx, eax
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm8, edx
	mov edx, r13d
	xor edx, r10d
	mov dword ptr [rsp + 128], edx
	mov ecx, eax
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	cvtsi2ss xmm9, edx
	mov edx, r13d
	xor edx, r12d
	mov dword ptr [rsp - 88], edx
	mov ecx, eax
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	xorps xmm3, xmm3
	cvtsi2ss xmm3, edx
	lea ecx, [r10 + 1002250642]
	xor r13d, ecx
	mov edx, eax
	xor edx, r13d
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	cvtsi2ss xmm12, esi
	mov esi, ebx
	xor esi, ebp
	mov dword ptr [rsp - 96], esi
	mov edx, eax
	xor edx, esi
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	cvtsi2ss xmm13, esi
	mov esi, ebx
	xor esi, r12d
	mov dword ptr [rsp - 112], esi
	mov edx, eax
	xor edx, esi
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	lea r11d, [rbx + 1136930381]
	cvtsi2ss xmm7, esi
	lea r14d, [rbx - 2021106534]
	mov r9d, ebx
	xor ebx, ecx
	mov edx, eax
	xor edx, ebx
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	cvtsi2ss xmm14, esi
	mov esi, r11d
	xor esi, ebp
	mov dword ptr [rsp + 144], esi
	mov edx, eax
	xor edx, esi
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	cvtsi2ss xmm15, esi
	mov esi, r11d
	xor esi, r12d
	mov dword ptr [rsp - 100], esi
	mov edx, eax
	xor edx, esi
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	cvtsi2ss xmm4, esi
	mov esi, r11d
	xor esi, ecx
	mov dword ptr [rsp - 104], esi
	mov edx, eax
	xor edx, esi
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	cvtsi2ss xmm11, esi
	movss dword ptr [rsp + 112], xmm11
	xor ebp, r14d
	mov dword ptr [rsp - 108], ebp
	mov edx, eax
	xor edx, ebp
	imul edx, edx, 668265261
	imul edx, edx
	mov esi, edx
	shl esi, 19
	xor esi, edx
	xor r12d, r14d
	mov edx, eax
	xor edx, r12d
	mov dword ptr [rsp - 76], r12d
	imul edx, edx, 668265261
	imul edx, edx
	mov edi, edx
	shl edi, 19
	xorps xmm11, xmm11
	cvtsi2ss xmm11, esi
	movss dword ptr [rsp - 80], xmm11
	xor edi, edx
	xorps xmm11, xmm11
	cvtsi2ss xmm11, edi
	subss xmm5, xmm10
	movss dword ptr [rsp + 108], xmm5
	subss xmm1, xmm2
	xor r9d, r10d
	mov dword ptr [rsp + 96], r9d
	xor r11d, r10d
	mov dword ptr [rsp + 72], r11d
	xor r10d, r14d
	mov qword ptr [rsp + 160], r10
	xor r14d, ecx
	mov dword ptr [rsp - 72], r14d
	mov dword ptr [rsp + 92], eax
	mov dword ptr [rsp + 100], eax
	mov dword ptr [rsp + 104], eax
	xor eax, r14d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 84], ecx
	mov edx, dword ptr [rsp - 116]
	mov eax, edx
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 92], ecx
	mov esi, dword ptr [rsp - 88]
	mov eax, esi
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp], ecx
	mov eax, r13d
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 4], ecx
	mov eax, r15d
	mov edi, dword ptr [rsp - 96]
	xor eax, edi
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 8], ecx
	mov eax, r15d
	xor eax, dword ptr [rsp - 112]
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 12], ecx
	mov eax, r15d
	xor eax, ebx
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 16], ecx
	mov r9d, dword ptr [rsp + 144]
	mov eax, r9d
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 20], ecx
	mov ebp, dword ptr [rsp - 100]
	mov eax, ebp
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 24], ecx
	mov r14d, dword ptr [rsp - 104]
	mov eax, r14d
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 28], ecx
	mov r8d, dword ptr [rsp - 108]
	mov eax, r8d
	xor eax, r15d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 32], ecx
	xor r12d, r15d
	imul eax, r12d, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	mov dword ptr [rsp - 36], ecx
	mov eax, dword ptr [rsp + 128]
	xor eax, r15d
	mov dword ptr [rsp + 76], eax
	mov eax, r11d
	xor eax, r15d
	mov dword ptr [rsp + 80], eax
	mov eax, r10d
	xor eax, r15d
	mov dword ptr [rsp + 84], eax
	lea eax, [r15 + 1720413743]
	lea r12d, [r15 - 854139810]
	mov dword ptr [rsp + 88], r15d
	mov r11d, dword ptr [rsp - 72]
	xor r15d, r11d
	imul ecx, r15d, 668265261
	imul ecx, ecx
	mov r10d, ecx
	shl r10d, 19
	xor r10d, ecx
	mov dword ptr [rsp - 40], r10d
	mov ecx, eax
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov r10d, ecx
	shl r10d, 19
	xor r10d, ecx
	mov dword ptr [rsp - 44], r10d
	mov ecx, eax
	xor ecx, esi
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp - 48], edx
	mov ecx, eax
	xor ecx, r13d
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp - 52], edx
	mov ecx, eax
	xor ecx, edi
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp - 56], edx
	mov ecx, eax
	mov r10d, dword ptr [rsp - 112]
	xor ecx, r10d
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp - 60], edx
	mov ecx, eax
	xor ecx, ebx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp - 64], edx
	mov ecx, eax
	xor ecx, r9d
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp - 68], edx
	mov ecx, eax
	xor ecx, ebp
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp + 52], edx
	mov ecx, eax
	xor ecx, r14d
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp + 48], edx
	mov ecx, eax
	xor ecx, r8d
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov edx, ecx
	shl edx, 19
	xor edx, ecx
	mov dword ptr [rsp + 44], edx
	mov ecx, eax
	mov edx, dword ptr [rsp - 76]
	xor ecx, edx
	imul ecx, ecx, 668265261
	imul ecx, ecx
	mov r8d, ecx
	shl r8d, 19
	xor r8d, ecx
	mov dword ptr [rsp + 40], r8d
	mov dword ptr [rsp + 56], eax
	mov dword ptr [rsp + 68], eax
	mov dword ptr [rsp + 60], eax
	mov dword ptr [rsp + 64], eax
	xor eax, r11d
	mov ecx, r11d
	imul eax, eax, 668265261
	imul eax, eax
	mov r15d, eax
	shl r15d, 19
	xor r15d, eax
	mov eax, dword ptr [rsp - 116]
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov r14d, eax
	shl r14d, 19
	xor r14d, eax
	mov eax, esi
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov ebp, eax
	shl ebp, 19
	xor ebp, eax
	xor r13d, r12d
	imul eax, r13d, 668265261
	imul eax, eax
	mov r13d, eax
	shl r13d, 19
	xor r13d, eax
	mov eax, edi
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov r11d, eax
	shl r11d, 19
	xor r11d, eax
	mov eax, r10d
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov r10d, eax
	shl r10d, 19
	xor r10d, eax
	xor ebx, r12d
	imul eax, ebx, 668265261
	imul eax, eax
	mov ebx, eax
	shl ebx, 19
	xor ebx, eax
	mov eax, r9d
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov r9d, eax
	shl r9d, 19
	xor r9d, eax
	mov eax, dword ptr [rsp - 100]
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov r8d, eax
	shl r8d, 19
	xor r8d, eax
	mov eax, dword ptr [rsp - 104]
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov edi, eax
	shl edi, 19
	xor edi, eax
	mov eax, dword ptr [rsp - 108]
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov esi, eax
	shl esi, 19
	xor esi, eax
	xor edx, r12d
	imul eax, edx, 668265261
	imul eax, eax
	mov edx, eax
	shl edx, 19
	xor edx, eax
	mov eax, ecx
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	cvtdq2ps xmm0, xmm0
	subss xmm6, xmm0
	movaps xmmword ptr [rsp + 144], xmm6
	movss xmm10, dword ptr [rip + .LCPI26_3]
	mulss xmm8, xmm10
	mulss xmm9, xmm10
	movss dword ptr [rsp + 12], xmm9
	mulss xmm3, xmm10
	mulss xmm12, xmm10
	subss xmm12, xmm3
	subss xmm3, xmm8
	subss xmm8, xmm9
	movss dword ptr [rsp + 8], xmm8
	subss xmm12, xmm8
	movss dword ptr [rsp + 4], xmm12
	mulss xmm3, xmm1
	movss dword ptr [rsp - 104], xmm3
	mulss xmm13, xmm10
	movss dword ptr [rsp + 16], xmm13
	mulss xmm7, xmm10
	mulss xmm14, xmm10
	subss xmm14, xmm7
	movss dword ptr [rsp + 20], xmm14
	subss xmm7, xmm13
	mulss xmm7, xmm1
	movss dword ptr [rsp - 88], xmm7
	mulss xmm15, xmm10
	movss dword ptr [rsp + 24], xmm15
	mulss xmm4, xmm10
	movss xmm0, dword ptr [rsp + 112]
	mulss xmm0, xmm10
	subss xmm0, xmm4
	movss dword ptr [rsp + 112], xmm0
	subss xmm4, xmm15
	mulss xmm4, xmm1
	movss dword ptr [rsp - 96], xmm4
	movss xmm0, dword ptr [rsp - 80]
	mulss xmm0, xmm10
	movss dword ptr [rsp - 80], xmm0
	mulss xmm11, xmm10
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 84]
	mulss xmm3, xmm10
	subss xmm3, xmm11
	movss dword ptr [rsp + 28], xmm3
	cvtsi2ss xmm5, dword ptr [rsp - 92]
	subss xmm11, xmm0
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp]
	mulss xmm11, xmm1
	movss dword ptr [rsp - 92], xmm11
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 4]
	mulss xmm5, xmm10
	mulss xmm0, xmm10
	mulss xmm3, xmm10
	subss xmm3, xmm0
	movss dword ptr [rsp + 36], xmm3
	subss xmm0, xmm5
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 8]
	mulss xmm0, xmm1
	movss dword ptr [rsp - 116], xmm0
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp - 12]
	mulss xmm3, xmm10
	movss dword ptr [rsp - 8], xmm3
	xorps xmm4, xmm4
	cvtsi2ss xmm4, dword ptr [rsp - 16]
	mulss xmm0, xmm10
	mulss xmm4, xmm10
	subss xmm4, xmm0
	movss dword ptr [rsp - 4], xmm4
	subss xmm0, xmm3
	mulss xmm0, xmm1
	movss dword ptr [rsp], xmm0
	xorps xmm4, xmm4
	cvtsi2ss xmm4, dword ptr [rsp - 20]
	mulss xmm4, xmm10
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp - 24]
	mulss xmm0, xmm10
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 28]
	mulss xmm3, xmm10
	subss xmm3, xmm0
	movss dword ptr [rsp + 32], xmm3
	subss xmm0, xmm4
	mulss xmm0, xmm1
	movss dword ptr [rsp - 84], xmm0
	cvtsi2ss xmm8, dword ptr [rsp - 32]
	mulss xmm8, xmm10
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 36]
	mulss xmm3, xmm10
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp - 40]
	mulss xmm0, xmm10
	subss xmm0, xmm3
	movss dword ptr [rsp - 76], xmm0
	cvtsi2ss xmm12, dword ptr [rsp - 44]
	subss xmm3, xmm8
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp - 48]
	mulss xmm3, xmm1
	movss dword ptr [rsp - 108], xmm3
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 52]
	mulss xmm12, xmm10
	mulss xmm0, xmm10
	mulss xmm3, xmm10
	subss xmm3, xmm0
	movss dword ptr [rsp - 72], xmm3
	subss xmm0, xmm12
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp - 56]
	mulss xmm0, xmm1
	movss dword ptr [rsp - 112], xmm0
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp - 60]
	mulss xmm3, xmm10
	movss dword ptr [rsp - 20], xmm3
	cvtsi2ss xmm6, dword ptr [rsp - 64]
	mulss xmm0, xmm10
	mulss xmm6, xmm10
	subss xmm6, xmm0
	movss dword ptr [rsp - 16], xmm6
	subss xmm0, xmm3
	mulss xmm0, xmm1
	movss dword ptr [rsp - 12], xmm0
	xorps xmm6, xmm6
	cvtsi2ss xmm6, dword ptr [rsp - 68]
	mulss xmm6, xmm10
	xorps xmm3, xmm3
	cvtsi2ss xmm3, dword ptr [rsp + 52]
	mulss xmm3, xmm10
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp + 48]
	mulss xmm0, xmm10
	subss xmm0, xmm3
	movss dword ptr [rsp - 68], xmm0
	subss xmm3, xmm6
	mulss xmm3, xmm1
	cvtsi2ss xmm13, dword ptr [rsp + 44]
	mulss xmm13, xmm10
	xorps xmm0, xmm0
	cvtsi2ss xmm0, dword ptr [rsp + 40]
	mulss xmm0, xmm10
	cvtsi2ss xmm7, r15d
	mulss xmm7, xmm10
	subss xmm7, xmm0
	movss dword ptr [rsp - 64], xmm7
	xorps xmm7, xmm7
	cvtsi2ss xmm7, r14d
	subss xmm0, xmm13
	cvtsi2ss xmm2, ebp
	mulss xmm0, xmm1
	movss dword ptr [rsp - 60], xmm0
	cvtsi2ss xmm9, r13d
	mulss xmm7, xmm10
	movss dword ptr [rsp - 52], xmm7
	mulss xmm2, xmm10
	mulss xmm9, xmm10
	subss xmm9, xmm2
	movss dword ptr [rsp - 44], xmm9
	subss xmm2, xmm7
	xorps xmm7, xmm7
	cvtsi2ss xmm7, r11d
	mulss xmm2, xmm1
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r10d
	mulss xmm7, xmm10
	movss dword ptr [rsp - 32], xmm7
	xorps xmm9, xmm9
	cvtsi2ss xmm9, ebx
	mulss xmm0, xmm10
	mulss xmm9, xmm10
	subss xmm9, xmm0
	movss dword ptr [rsp - 28], xmm9
	subss xmm0, xmm7
	mulss xmm0, xmm1
	movss dword ptr [rsp - 24], xmm0
	xorps xmm7, xmm7
	cvtsi2ss xmm7, r9d
	mulss xmm7, xmm10
	movss dword ptr [rsp - 56], xmm7
	cvtsi2ss xmm11, r8d
	mulss xmm11, xmm10
	xorps xmm0, xmm0
	cvtsi2ss xmm0, edi
	mulss xmm0, xmm10
	subss xmm0, xmm11
	movss dword ptr [rsp - 48], xmm0
	subss xmm11, xmm7
	mulss xmm11, xmm1
	xorps xmm9, xmm9
	cvtsi2ss xmm9, esi
	mulss xmm9, xmm10
	movss dword ptr [rsp - 40], xmm9
	xorps xmm0, xmm0
	cvtsi2ss xmm0, edx
	mulss xmm0, xmm10
	xorps xmm7, xmm7
	cvtsi2ss xmm7, ecx
	mulss xmm7, xmm10
	subss xmm7, xmm0
	movss dword ptr [rsp - 36], xmm7
	subss xmm0, xmm9
	mulss xmm0, xmm1
	movss dword ptr [rsp - 100], xmm0
	movaps xmm9, xmm1
	movaps xmm14, xmm1
	mulss xmm14, xmm1
	mulss xmm9, xmm14
	movaps xmm0, xmm9
	movss xmm7, dword ptr [rsp + 4]
	mulss xmm0, xmm7
	movss xmm1, dword ptr [rsp + 8]
	subss xmm1, xmm7
	mulss xmm1, xmm14
	addss xmm1, xmm0
	movss xmm0, dword ptr [rsp - 104]
	addss xmm0, xmm1
	addss xmm0, dword ptr [rsp + 12]
	movss dword ptr [rsp - 104], xmm0
	mov edi, dword ptr [rsp + 96]
	mov eax, dword ptr [rsp + 92]
	xor eax, edi
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm1, xmm1
	cvtsi2ss xmm1, ecx
	mulss xmm1, xmm10
	movss xmm0, dword ptr [rsp + 16]
	subss xmm0, xmm1
	movss xmm15, dword ptr [rsp + 20]
	subss xmm15, xmm0
	movaps xmm7, xmm9
	mulss xmm7, xmm15
	subss xmm0, xmm15
	mulss xmm0, xmm14
	addss xmm0, xmm7
	mov esi, dword ptr [rsp + 72]
	mov eax, dword ptr [rsp + 100]
	xor eax, esi
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm7, xmm7
	cvtsi2ss xmm7, ecx
	movss xmm15, dword ptr [rsp - 88]
	addss xmm15, xmm0
	addss xmm15, xmm1
	movss dword ptr [rsp - 88], xmm15
	mulss xmm7, xmm10
	movss xmm1, dword ptr [rsp + 24]
	subss xmm1, xmm7
	movss xmm15, dword ptr [rsp + 112]
	subss xmm15, xmm1
	movaps xmm0, xmm9
	mulss xmm0, xmm15
	subss xmm1, xmm15
	mulss xmm1, xmm14
	addss xmm1, xmm0
	movss xmm15, dword ptr [rsp - 96]
	addss xmm15, xmm1
	mov r8, qword ptr [rsp + 160]
	mov eax, dword ptr [rsp + 104]
	xor eax, r8d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	addss xmm15, xmm7
	movss dword ptr [rsp - 96], xmm15
	mulss xmm0, xmm10
	movss xmm1, dword ptr [rsp - 80]
	subss xmm1, xmm0
	movss xmm15, dword ptr [rsp + 28]
	subss xmm15, xmm1
	movaps xmm7, xmm9
	mulss xmm7, xmm15
	subss xmm1, xmm15
	mulss xmm1, xmm14
	addss xmm1, xmm7
	movss xmm7, dword ptr [rsp - 92]
	addss xmm7, xmm1
	addss xmm7, xmm0
	movss dword ptr [rsp - 92], xmm7
	imul eax, dword ptr [rsp + 76], 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	mulss xmm0, xmm10
	subss xmm5, xmm0
	movss xmm1, dword ptr [rsp + 36]
	subss xmm1, xmm5
	movaps xmm7, xmm9
	mulss xmm7, xmm1
	subss xmm5, xmm1
	mulss xmm5, xmm14
	addss xmm5, xmm7
	movss xmm1, dword ptr [rsp - 116]
	addss xmm1, xmm5
	imul eax, dword ptr [rsp + 80], 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm5, xmm5
	cvtsi2ss xmm5, ecx
	addss xmm1, xmm0
	movss dword ptr [rsp - 116], xmm1
	mulss xmm5, xmm10
	subss xmm4, xmm5
	movss xmm1, dword ptr [rsp + 32]
	subss xmm1, xmm4
	movaps xmm0, xmm9
	mulss xmm0, xmm1
	subss xmm4, xmm1
	mulss xmm4, xmm14
	addss xmm4, xmm0
	movss xmm0, dword ptr [rsp - 84]
	addss xmm0, xmm4
	addss xmm0, xmm5
	movss dword ptr [rsp - 84], xmm0
	imul eax, dword ptr [rsp + 84], 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	mulss xmm0, xmm10
	subss xmm8, xmm0
	movss xmm1, dword ptr [rsp - 76]
	subss xmm1, xmm8
	movaps xmm4, xmm9
	mulss xmm4, xmm1
	subss xmm8, xmm1
	mulss xmm8, xmm14
	addss xmm8, xmm4
	mov edx, dword ptr [rsp + 128]
	mov eax, dword ptr [rsp + 56]
	xor eax, edx
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	movss xmm5, dword ptr [rsp - 108]
	addss xmm5, xmm8
	addss xmm5, xmm0
	movaps xmm8, xmm5
	mulss xmm4, xmm10
	subss xmm12, xmm4
	movss xmm1, dword ptr [rsp - 72]
	subss xmm1, xmm12
	movaps xmm0, xmm9
	mulss xmm0, xmm1
	subss xmm12, xmm1
	mulss xmm12, xmm14
	addss xmm12, xmm0
	movss xmm1, dword ptr [rsp - 112]
	addss xmm1, xmm12
	mov eax, dword ptr [rsp + 60]
	xor eax, esi
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	addss xmm1, xmm4
	movss dword ptr [rsp - 112], xmm1
	mulss xmm0, xmm10
	subss xmm6, xmm0
	movss xmm1, dword ptr [rsp - 68]
	subss xmm1, xmm6
	movaps xmm4, xmm9
	mulss xmm4, xmm1
	subss xmm6, xmm1
	mulss xmm6, xmm14
	addss xmm6, xmm4
	addss xmm3, xmm6
	addss xmm3, xmm0
	mov eax, dword ptr [rsp + 64]
	xor eax, r8d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	mulss xmm0, xmm10
	subss xmm13, xmm0
	movss xmm1, dword ptr [rsp - 64]
	subss xmm1, xmm13
	movaps xmm4, xmm9
	mulss xmm4, xmm1
	subss xmm13, xmm1
	mulss xmm13, xmm14
	addss xmm13, xmm4
	movss xmm15, dword ptr [rsp - 60]
	addss xmm15, xmm13
	addss xmm15, xmm0
	mov eax, edx
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	mulss xmm0, xmm10
	movss xmm1, dword ptr [rsp - 52]
	subss xmm1, xmm0
	movss xmm6, dword ptr [rsp - 44]
	subss xmm6, xmm1
	movaps xmm4, xmm9
	mulss xmm4, xmm6
	subss xmm1, xmm6
	mulss xmm1, xmm14
	addss xmm1, xmm4
	mov eax, esi
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm4, xmm4
	cvtsi2ss xmm4, ecx
	movaps xmm5, xmm2
	addss xmm5, xmm1
	addss xmm5, xmm0
	movss dword ptr [rsp - 80], xmm5
	mulss xmm4, xmm10
	movss xmm1, dword ptr [rsp - 56]
	subss xmm1, xmm4
	movss xmm6, dword ptr [rsp - 48]
	subss xmm6, xmm1
	movaps xmm0, xmm9
	mulss xmm0, xmm6
	subss xmm1, xmm6
	mulss xmm1, xmm14
	addss xmm1, xmm0
	addss xmm11, xmm1
	mov rax, r8
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ecx
	addss xmm11, xmm4
	mulss xmm0, xmm10
	movss xmm1, dword ptr [rsp - 40]
	subss xmm1, xmm0
	movss xmm6, dword ptr [rsp - 36]
	subss xmm6, xmm1
	movaps xmm7, xmm9
	movaps xmmword ptr [rsp + 128], xmm9
	movaps xmmword ptr [rsp + 112], xmm9
	mulss xmm9, xmm6
	subss xmm1, xmm6
	mulss xmm1, xmm14
	addss xmm1, xmm9
	movss xmm2, dword ptr [rsp - 100]
	addss xmm2, xmm1
	addss xmm2, xmm0
	movss xmm1, dword ptr [rsp - 96]
	movss xmm4, dword ptr [rsp - 92]
	subss xmm4, xmm1
	movss xmm12, dword ptr [rsp - 104]
	subss xmm1, xmm12
	movss xmm13, dword ptr [rsp - 88]
	subss xmm12, xmm13
	subss xmm4, xmm12
	movaps xmm6, xmmword ptr [rsp + 144]
	mulss xmm1, xmm6
	movss xmm9, dword ptr [rsp - 84]
	subss xmm8, xmm9
	movss dword ptr [rsp - 108], xmm8
	subss xmm9, dword ptr [rsp - 116]
	mulss xmm9, xmm6
	subss xmm15, xmm3
	subss xmm3, dword ptr [rsp - 112]
	mulss xmm3, xmm6
	subss xmm2, xmm11
	movss dword ptr [rsp - 100], xmm2
	subss xmm11, xmm5
	mulss xmm11, xmm6
	movaps xmm0, xmm6
	mulss xmm6, xmm6
	mulss xmm0, xmm6
	movaps xmm8, xmm0
	mulss xmm8, xmm4
	subss xmm12, xmm4
	mulss xmm12, xmm6
	addss xmm12, xmm8
	addss xmm1, xmm12
	addss xmm1, xmm13
	mov eax, dword ptr [rsp + 88]
	xor eax, edi
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm8, xmm8
	cvtsi2ss xmm8, ecx
	mulss xmm8, xmm10
	movss xmm12, dword ptr [rsp - 8]
	subss xmm12, xmm8
	movss xmm13, dword ptr [rsp - 4]
	subss xmm13, xmm12
	mulss xmm7, xmm13
	subss xmm12, xmm13
	mulss xmm12, xmm14
	addss xmm12, xmm7
	movss xmm7, dword ptr [rsp]
	addss xmm7, xmm12
	addss xmm7, xmm8
	movss xmm8, dword ptr [rsp - 116]
	subss xmm8, xmm7
	movaps xmm12, xmm7
	movss xmm4, dword ptr [rsp - 108]
	subss xmm4, xmm8
	movaps xmm7, xmm0
	mulss xmm7, xmm4
	subss xmm8, xmm4
	mulss xmm8, xmm6
	addss xmm8, xmm7
	mov eax, dword ptr [rsp + 68]
	xor eax, edi
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm7, xmm7
	cvtsi2ss xmm7, ecx
	addss xmm9, xmm8
	addss xmm9, xmm12
	mulss xmm7, xmm10
	movss xmm8, dword ptr [rsp - 20]
	subss xmm8, xmm7
	movss xmm12, dword ptr [rsp - 16]
	subss xmm12, xmm8
	movaps xmm2, xmmword ptr [rsp + 128]
	mulss xmm2, xmm12
	subss xmm8, xmm12
	mulss xmm8, xmm14
	addss xmm8, xmm2
	movss xmm5, dword ptr [rsp - 12]
	addss xmm5, xmm8
	addss xmm5, xmm7
	movss xmm7, dword ptr [rsp - 112]
	subss xmm7, xmm5
	movaps xmm8, xmm5
	subss xmm15, xmm7
	movaps xmm5, xmm0
	mulss xmm5, xmm15
	subss xmm7, xmm15
	mulss xmm7, xmm6
	addss xmm7, xmm5
	addss xmm3, xmm7
	mov eax, edi
	xor eax, r12d
	imul eax, eax, 668265261
	imul eax, eax
	mov ecx, eax
	shl ecx, 19
	xor ecx, eax
	xorps xmm5, xmm5
	cvtsi2ss xmm5, ecx
	addss xmm3, xmm8
	mulss xmm5, xmm10
	movss xmm7, dword ptr [rsp - 32]
	subss xmm7, xmm5
	movss xmm2, dword ptr [rsp - 28]
	subss xmm2, xmm7
	movaps xmm4, xmmword ptr [rsp + 112]
	mulss xmm4, xmm2
	subss xmm7, xmm2
	mulss xmm7, xmm14
	addss xmm7, xmm4
	movss xmm2, dword ptr [rsp - 24]
	addss xmm2, xmm7
	addss xmm2, xmm5
	movss xmm5, dword ptr [rsp - 80]
	subss xmm5, xmm2
	movss xmm4, dword ptr [rsp - 100]
	subss xmm4, xmm5
	mulss xmm0, xmm4
	subss xmm5, xmm4
	mulss xmm5, xmm6
	addss xmm5, xmm0
	addss xmm11, xmm5
	addss xmm11, xmm2
	subss xmm11, xmm3
	subss xmm3, xmm1
	subss xmm1, xmm9
	subss xmm11, xmm1
	movss xmm2, dword ptr [rsp + 108]
	movaps xmm0, xmm2
	mulss xmm3, xmm2
	mulss xmm2, xmm2
	mulss xmm0, xmm2
	mulss xmm0, xmm11
	subss xmm1, xmm11
	mulss xmm1, xmm2
	addss xmm1, xmm0
	addss xmm3, xmm1
	addss xmm3, xmm9
	mulss xmm3, dword ptr [rip + .LCPI26_4]
	movaps xmm0, xmm3
	add rsp, 168
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret