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
	movd ecx, xmm2
	lea r10d, [rcx - 1]
	inc ecx
	xorps xmm0, xmm0
	cmp r10d, ecx
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
	lea r15d, [rax - 1]
	lea esi, [rax + 1]
	cmp r15d, esi
	jle .LBB_4
.LBB_2:
	xor eax, eax
	cmp r10d, ecx
	setl dl
	jge .LBB_20
	mov al, dl
	add r10d, eax
	cmp r10d, ecx
	jle .LBB_2
	jmp .LBB_20
.LBB_4:
	pshufd xmm2, xmm2, 238
	movd edx, xmm2
	lea edi, [rdx - 1]
	lea r8d, [rdx + 1]
	mov dword ptr [rsp - 12], edi
	cmp edi, r8d
	jle .LBB_5
.LBB_15:
	mov eax, r15d
.LBB_16:
	xor edx, edx
	cmp eax, esi
	setl dil
	jge .LBB_18
	mov dl, dil
	add eax, edx
	cmp eax, esi
	jle .LBB_16
.LBB_18:
	xor eax, eax
	cmp r10d, ecx
	setl dl
	jge .LBB_20
	mov al, dl
	add r10d, eax
	cmp r10d, ecx
	jle .LBB_15
	jmp .LBB_20
.LBB_5:
	imul r9d, r10d, 501125321
	imul eax, eax, 1136930381
	add eax, -1136930381
	mov dword ptr [rsp - 24], eax
	imul r11d, edx, 1720413743
	add r11d, -1720413743
	xor ebx, ebx
	movss xmm2, dword ptr [rip + .LCPI12_2]
	lea r14, [rip + .L__unnamed__0]
	movaps xmm0, xmmword ptr [rip + .LCPI12_3]
	mov qword ptr [rsp - 8], rcx
	mov dword ptr [rsp - 20], r15d
.LBB_6:
	mov dword ptr [rsp - 16], r10d
	xorps xmm3, xmm3
	cvtsi2ss xmm3, r10d
	mov ecx, dword ptr [rsp - 24]
.LBB_7:
	mov ebp, ecx
	xor ebp, r9d
	xorps xmm4, xmm4
	cvtsi2ss xmm4, r15d
	movlhps xmm4, xmm3
	shufps xmm4, xmm3, 226
	mov r13d, r11d
	mov edi, dword ptr [rsp - 12]
.LBB_8:
	movaps xmm5, xmm2
	xor edx, edx
	cmp edi, r8d
	setl al
	mov r12d, ebp
	xor r12d, r13d
	imul r12d, r12d, 668265261
	xorps xmm2, xmm2
	cvtsi2ss xmm2, edi
	mov r10d, r12d
	and r10d, 1020
	movaps xmm6, xmm4
	shufps xmm6, xmm2, 4
	subps xmm6, xmm1
	movaps xmm2, xmmword ptr [r14 + 4*r10]
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
	mov dl, al
	add edi, edx
	add r13d, 1720413743
	cmp edi, r8d
	jle .LBB_8
.LBB_10:
	xor eax, eax
	cmp r15d, esi
	setl dl
	jge .LBB_12
	mov al, dl
	add r15d, eax
	add ecx, 1136930381
	cmp r15d, esi
	jle .LBB_7
.LBB_12:
	xor eax, eax
	mov rcx, qword ptr [rsp - 8]
	mov r10d, dword ptr [rsp - 16]
	cmp r10d, ecx
	setl dl
	mov r15d, dword ptr [rsp - 20]
	jge .LBB_14
	mov al, dl
	add r10d, eax
	add r9d, 501125321
	cmp r10d, ecx
	jle .LBB_6
.LBB_14:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ebx
	mulss xmm0, dword ptr [rip + .LCPI12_4]
	jmp .LBB_20