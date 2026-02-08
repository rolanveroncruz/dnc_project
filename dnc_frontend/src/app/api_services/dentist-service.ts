// src/app/api_services/dentist-service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import {LoginService} from '../login.service';

/**
 * Mirrors your Rust DentistWithLookups struct exactly.
 * (All fields + lookup-name aliases)
 */
export interface DentistWithLookups {
    // ---- Dentist columns
    id: number;
    last_name: string;
    given_name: string;
    middle_name: string | null;
    email: string | null;
    retainer_fee: number;
    dentist_status_id: number | null;
    dentist_history_id: number | null;
    dentist_requested_by: string | null;
    accre_dentist_contract_id: number | null;
    accre_document_code: string | null;
    accreditation_date: string | null;
    accre_contract_sent_date: string | null;
    accre_contract_file_path: string | null;
    acc_tin: string | null;
    acc_bank_name: string | null;
    acc_account_name: string | null;
    acc_account_number: string | null;
    acc_tax_type_id: number | null;
    acc_tax_classification_id: number | null;

    // ---- Lookup names (aliased in the query via expr_as)
    dentist_contract_name: string | null;
    dentist_history_name: string | null;
    dentist_status_name: string | null;
    tax_type_name: string | null;
    tax_classification_name: string | null;
}

/**
 * Optional: used later if you add POST/PATCH dentists.
 * Keeping it here is convenient even if unused today.
 */
export interface CreateDentistBody {
    last_name: string;
    given_name: string;
    middle_name?: string | null;
    email?: string | null;
    retainer_fee: number;

    dentist_status_id?: number | null;
    dentist_history_id?: number | null;
    dentist_requested_by?: string | null;

    accre_dentist_contract_id?: number | null;
    accre_document_code?: string | null;
    accreditation_date?: string | null;
    accre_contract_sent_date?: string | null;
    accre_contract_file_path?: string | null;

    acc_tin?: string | null;
    acc_bank_name?: string | null;
    acc_account_name?: string | null;
    acc_account_number?: string | null;

    acc_tax_type_id?: number | null;
    acc_tax_classification_id?: number | null;
}

export type PatchDentistBody = Partial<CreateDentistBody>;

@Injectable({ providedIn: 'root' })
export class DentistService {
    private http = inject(HttpClient);
    private loginService = inject(LoginService);

    // Adjust this if your backend uses a different base path
    // (matches your existing services style)
    private readonly baseUrl = `${environment.apiUrl}/api/dentists/`;
    private authHeaders(): HttpHeaders {
        const token = this.loginService.token?.() ?? '';
        return new HttpHeaders({ Authorization: `Bearer ${token}` });
    }

    /**
     * GET /dentists
     * Returns: DentistWithLookups[]
     */
    getAllDentists(): Observable<DentistWithLookups[]> {
        return this.http.get<DentistWithLookups[]>(this.baseUrl, { headers: this.authHeaders() });
    }

    /**
     * GET /dentists/{id}
     * Returns: DentistWithLookups
     */
    getDentistById(id: number): Observable<DentistWithLookups> {
        return this.http.get<DentistWithLookups>(`${this.baseUrl}${id}`, { headers: this.authHeaders() });
    }

    // ------------------------------------------------------------
    // Optional future endpoints (only enable when your backend has them)
    // ------------------------------------------------------------

    createDentist(body: CreateDentistBody): Observable<DentistWithLookups> {
      return this.http.post<DentistWithLookups>(`${this.baseUrl}`, body, { headers: this.authHeaders() });
    }

    patchDentist(id: number, body: PatchDentistBody): Observable<DentistWithLookups> {
      return this.http.patch<DentistWithLookups>(`${this.baseUrl}${id}`, body, { headers: this.authHeaders() });
    }
}
