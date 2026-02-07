import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders, HttpResponse } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { LoginService } from '../login.service';

export interface StoredDocumentMeta {
    id: number;
    file_name: string;
    content_type: string;
    size_bytes: number;
    updated_at: string; // ISO string
}

@Injectable({ providedIn: 'root' })
export class SingleDocumentUploadService {
    private readonly http = inject(HttpClient);
    private readonly loginService = inject(LoginService);

    private readonly baseUrl = `${environment.apiUrl}/api`;

    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    /**
     * Change these endpoints to match your backend.
     * This version assumes one doc “slot” per dentist:
     *  - GET    /dentists/:id/document            -> meta or 404
     *  - GET    /dentists/:id/document/download   -> streams file
     *  - PUT    /dentists/:id/document            -> multipart upload/replace
     *  - DELETE /dentists/:id/document            -> delete
     */

    getMeta(dentistId: number): Observable<StoredDocumentMeta> {
        return this.http.get<StoredDocumentMeta>(
            `${this.baseUrl}/dentists/${dentistId}/document`,
            { headers: this.authHeaders() }
        );
    }

    uploadReplace(dentistId: number, file: File): Observable<StoredDocumentMeta> {
        const form = new FormData();
        form.append('file', file, file.name);

        return this.http.put<StoredDocumentMeta>(
            `${this.baseUrl}/dentists/${dentistId}/document`,
            form,
            { headers: this.authHeaders() }
        );
    }

    delete(dentistId: number): Observable<void> {
        return this.http.delete<void>(
            `${this.baseUrl}/dentists/${dentistId}/document`,
            { headers: this.authHeaders() }
        );
    }

    /**
     * Download via Bearer header (works even if your download endpoint is protected).
     * This returns a Blob + attempts to infer filename from Content-Disposition.
     */
    download(dentistId: number): Observable<{ blob: Blob; filename: string }> {
        return new Observable((subscriber) => {
            this.http.get(
                `${this.baseUrl}/dentists/${dentistId}/document/download`,
                {
                    headers: this.authHeaders(),
                    responseType: 'blob',
                    observe: 'response',
                }
            ).subscribe({
                next: (resp: HttpResponse<Blob>) => {
                    const blob = resp.body ?? new Blob();
                    const cd = resp.headers.get('content-disposition') ?? '';
                    const filename = parseFilenameFromContentDisposition(cd) ?? 'document';
                    subscriber.next({ blob, filename });
                    subscriber.complete();
                },
                error: (err) => subscriber.error(err),
            });
        });
    }
}

function parseFilenameFromContentDisposition(cd: string): string | null {
    // Typical: attachment; filename="abc.pdf"
    const match = /filename\*?=(?:UTF-8''|")?([^\";]+)"?/i.exec(cd);
    if (!match) return null;
    try {
        return decodeURIComponent(match[1]);
    } catch {
        return match[1];
    }
}
