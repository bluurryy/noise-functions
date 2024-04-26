inspect_asm::cell_value_3d:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	movss xmm1, dword ptr [rdi]
	xorps xmm2, xmm2
	cmpnless xmm2, xmm1
	movss xmm0, dword ptr [rip + .LCPI10_0]
	movaps xmm3, xmm2
	andps xmm3, xmm0
	movss xmm4, dword ptr [rip + .LCPI10_1]
	andnps xmm2, xmm4
	orps xmm2, xmm3
	addss xmm2, xmm1
	cvttss2si r9d, xmm2
	movss xmm5, dword ptr [rip + .LCPI10_2]
	ucomiss xmm2, xmm5
	mov eax, 2147483647
	cmova r9d, eax
	xor edx, edx
	ucomiss xmm2, xmm2
	cmovp r9d, edx
	movss xmm2, dword ptr [rdi + 4]
	xorps xmm6, xmm6
	cmpnless xmm6, xmm2
	movaps xmm3, xmm6
	andps xmm3, xmm0
	andnps xmm6, xmm4
	orps xmm6, xmm3
	addss xmm6, xmm2
	cvttss2si ecx, xmm6
	ucomiss xmm6, xmm5
	cmova ecx, eax
	movss xmm3, dword ptr [rdi + 8]
	ucomiss xmm6, xmm6
	cmovp ecx, edx
	xorps xmm6, xmm6
	cmpnless xmm6, xmm3
	andps xmm0, xmm6
	andnps xmm6, xmm4
	orps xmm6, xmm0
	addss xmm6, xmm3
	cvttss2si edi, xmm6
	ucomiss xmm6, xmm5
	cmova edi, eax
	ucomiss xmm6, xmm6
	cmovp edi, edx
	xorps xmm0, xmm0
	lea r10d, [r9 - 1]
	inc r9d
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
	lea r13d, [rcx - 1]
	inc ecx
	cmp r13d, ecx
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
	lea r8d, [rdi - 1]
	inc edi
	cmp r8d, edi
	jle .LBB_5
.LBB_15:
	xor eax, eax
	cmp r10d, r9d
	setl dl
	mov esi, r13d
.LBB_16:
	xor edi, edi
	cmp esi, ecx
	setl r8b
	jge .LBB_18
	mov dil, r8b
	add esi, edi
	cmp esi, ecx
	jle .LBB_16
.LBB_18:
	cmp r10d, r9d
	jge .LBB_20
	mov al, dl
	add r10d, eax
	cmp r10d, r9d
	jle .LBB_15
	jmp .LBB_20
.LBB_5:
	imul eax, r10d, 501125321
	mov dword ptr [rsp - 28], eax
	imul eax, r13d, 1136930381
	mov dword ptr [rsp - 24], eax
	imul eax, r8d, 1720413743
	mov dword ptr [rsp - 12], eax
	movss xmm5, dword ptr [rip + .LCPI10_3]
	xor ebx, ebx
	lea r14, [rip + .L__unnamed__0]
	movss xmm0, dword ptr [rip + .LCPI10_4]
	movaps xmm4, xmmword ptr [rip + .LCPI10_5]
	mov qword ptr [rsp - 8], r9
	mov dword ptr [rsp - 20], r13d
.LBB_6:
	cmp r10d, r9d
	mov dword ptr [rsp - 16], r10d
	xorps xmm6, xmm6
	cvtsi2ss xmm6, r10d
	setl byte ptr [rsp - 30]
	subss xmm6, xmm1
	mov r15d, dword ptr [rsp - 24]
.LBB_7:
	xor r10d, r10d
	cmp r13d, ecx
	setl byte ptr [rsp - 29]
	mov r12d, r15d
	xorps xmm7, xmm7
	cvtsi2ss xmm7, r13d
	xor r12d, dword ptr [rsp - 28]
	subss xmm7, xmm2
	mov r11d, dword ptr [rsp - 12]
	mov esi, r8d
.LBB_8:
	movaps xmm8, xmm5
	xor edx, edx
	cmp r8d, edi
	setl al
	mov ebp, r12d
	xor ebp, r11d
	imul ebp, ebp, 668265261
	mov r9d, ebp
	and r9d, 1020
	movss xmm9, dword ptr [r14 + 4*r9 + 4]
	mulss xmm9, xmm0
	xorps xmm5, xmm5
	cvtsi2ss xmm5, r8d
	addss xmm9, xmm7
	subss xmm5, xmm3
	mulss xmm9, xmm9
	movss xmm10, dword ptr [r14 + 4*r9 + 8]
	movss xmm11, dword ptr [r14 + 4*r9]
	unpcklps xmm11, xmm10
	mulps xmm11, xmm4
	movlhps xmm5, xmm6
	shufps xmm5, xmm6, 226
	addps xmm5, xmm11
	mulps xmm5, xmm5
	addss xmm9, xmm5
	shufps xmm5, xmm5, 85
	addss xmm5, xmm9
	ucomiss xmm8, xmm5
	cmova ebx, ebp
	minss xmm5, xmm8
	cmp r8d, edi
	jge .LBB_10
	mov dl, al
	add r8d, edx
	add r11d, 1720413743
	cmp r8d, edi
	jle .LBB_8
.LBB_10:
	cmp r13d, ecx
	mov r8d, esi
	jge .LBB_12
	movzx eax, byte ptr [rsp - 29]
	mov r10b, al
	add r13d, r10d
	add r15d, 1136930381
	cmp r13d, ecx
	jle .LBB_7
.LBB_12:
	mov r9, qword ptr [rsp - 8]
	mov r10d, dword ptr [rsp - 16]
	cmp r10d, r9d
	mov r13d, dword ptr [rsp - 20]
	jge .LBB_14
	xor eax, eax
	movzx edx, byte ptr [rsp - 30]
	mov al, dl
	add r10d, eax
	add dword ptr [rsp - 28], 501125321
	cmp r10d, r9d
	jle .LBB_6
.LBB_14:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, ebx
	mulss xmm0, dword ptr [rip + .LCPI10_6]
	jmp .LBB_20