inspect_asm::cell_value_3d_simd:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	movaps xmm1, xmmword ptr [rdi]
	xorps xmm0, xmm0
	cmpleps xmm0, xmm1
	movaps xmm2, xmm0
	andnps xmm2, xmmword ptr [rip + .LCPI12_0]
	andps xmm0, xmmword ptr [rip + .LCPI12_1]
	orps xmm0, xmm2
	addps xmm0, xmm1
	cvttps2dq xmm2, xmm0
	movd r9d, xmm2
	lea r10d, [r9 - 1]
	inc r9d
	xorps xmm0, xmm0
	cmp r10d, r9d
	jle .LBB_1
.LBB_20:
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
.LBB_1:
	pshufd xmm3, xmm2, 85
	movd eax, xmm3
	lea r13d, [rax - 1]
	lea esi, [rax + 1]
	cmp r13d, esi
	jle .LBB_4
.LBB_2:
	xor eax, eax
	cmp r10d, r9d
	setl cl
	jge .LBB_20
	mov al, cl
	add r10d, eax
	cmp r10d, r9d
	jle .LBB_2
	jmp .LBB_20
.LBB_4:
	pshufd xmm2, xmm2, 238
	movd ecx, xmm2
	lea edx, [rcx - 1]
	lea r8d, [rcx + 1]
	mov dword ptr [rsp - 12], edx
	cmp edx, r8d
	jle .LBB_5
.LBB_15:
	xor eax, eax
	cmp r10d, r9d
	setl cl
	mov edx, r13d
.LBB_16:
	xor edi, edi
	cmp edx, esi
	setl r8b
	jge .LBB_18
	mov dil, r8b
	add edx, edi
	cmp edx, esi
	jle .LBB_16
.LBB_18:
	cmp r10d, r9d
	jge .LBB_20
	mov al, cl
	add r10d, eax
	cmp r10d, r9d
	jle .LBB_15
	jmp .LBB_20
.LBB_5:
	imul edx, r10d, 501125321
	mov dword ptr [rsp - 32], edx
	imul eax, eax, 1136930381
	add eax, -1136930381
	mov dword ptr [rsp - 28], eax
	imul eax, ecx, 1720413743
	add eax, -1720413743
	mov dword ptr [rsp - 16], eax
	movss xmm2, dword ptr [rip + .LCPI12_2]
	xor ebx, ebx
	lea r14, [rip + .L__unnamed__0]
	movaps xmm0, xmmword ptr [rip + .LCPI12_3]
	mov qword ptr [rsp - 8], r9
	mov dword ptr [rsp - 24], r13d
.LBB_6:
	xorps xmm3, xmm3
	cvtsi2ss xmm3, r10d
	mov dword ptr [rsp - 20], r10d
	cmp r10d, r9d
	setl byte ptr [rsp - 33]
	mov ebp, dword ptr [rsp - 28]
.LBB_7:
	xor r10d, r10d
	cmp r13d, esi
	setl dl
	mov r15d, ebp
	xor r15d, dword ptr [rsp - 32]
	xorps xmm4, xmm4
	cvtsi2ss xmm4, r13d
	movlhps xmm4, xmm3
	shufps xmm4, xmm3, 226
	mov r11d, dword ptr [rsp - 16]
	mov edi, dword ptr [rsp - 12]
.LBB_8:
	movaps xmm5, xmm2
	xor ecx, ecx
	cmp edi, r8d
	setl al
	mov r12d, r15d
	xor r12d, r11d
	imul r12d, r12d, 668265261
	xorps xmm2, xmm2
	cvtsi2ss xmm2, edi
	mov r9d, r12d
	and r9d, 1020
	movaps xmm6, xmm4
	shufps xmm6, xmm2, 4
	subps xmm6, xmm1
	movaps xmm2, xmmword ptr [r14 + 4*r9]
	mulps xmm2, xmm0
	addps xmm2, xmm6
	mulps xmm2, xmm2
	movaps xmm6, xmm2
	shufps xmm6, xmm2, 85
	addss xmm6, xmm2
	movhlps xmm2, xmm2
	addss xmm2, xmm6
	ucomiss xmm5, xmm2
	cmova ebx, r12d
	minss xmm2, xmm5
	cmp edi, r8d
	jge .LBB_10
	mov cl, al
	add edi, ecx
	add r11d, 1720413743
	cmp edi, r8d
	jle .LBB_8
.LBB_10:
	cmp r13d, esi
	jge .LBB_12
	mov r10b, dl
	add r13d, r10d
	add ebp, 1136930381
	cmp r13d, esi
	jle .LBB_7
.LBB_12:
	mov r9, qword ptr [rsp - 8]
	mov r10d, dword ptr [rsp - 20]
	cmp r10d, r9d
	mov r13d, dword ptr [rsp - 24]
	jge .LBB_14
	xor eax, eax
	movzx ecx, byte ptr [rsp - 33]
	mov al, cl
	add r10d, eax
	add dword ptr [rsp - 32], 501125321
	cmp r10d, r9d
	jle .LBB_6
.LBB_14:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ebx
	mulss xmm0, dword ptr [rip + .LCPI12_4]
	jmp .LBB_20