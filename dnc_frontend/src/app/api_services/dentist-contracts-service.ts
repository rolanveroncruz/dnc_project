// src/app/api_services/dentist-contracts-service.ts
import { Injectable, inject } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, catchError, throwError } from 'rxjs';
import { environment } from '../../environments/environment';
import { LoginService } from '../login.service';

// --- Response shapes ---
export interface DentistContractRow {
  id: number;
  name: string;
  description: string;
  active: boolean;
  last_modified_by: string;
  last_modified_on: string;
}

export interface DentistContractServiceRateRow {
  // CHANGE: support either backend naming style, so we don't break if backend says dental_service_id
  service_id?: number;
  dental_service_id?: number;

  rate: number;
}

export interface DentistContractWithRates {
  contract: DentistContractRow;
  rates: DentistContractServiceRateRow[];
}

// --- Request shapes ---
export interface DentistContractRateInput {
  // CHANGE: page will send service_id (cleaner). If your backend expects dental_service_id, see note below.
  service_id: number;
  rate: number;
}

export interface CreateDentistContractRequest {
  name: string;
  description: string;
  active: boolean;
  rates?: DentistContractRateInput[];
}

export interface PatchDentistContractRequest {
  name?: string | null;
  description?: string | null;
  active?: boolean | null;
  rates?: DentistContractRateInput[];
}




@Injectable({ providedIn: 'root' })
export class DentistContractsService {
  private readonly http = inject(HttpClient);
  private readonly loginService = inject(LoginService);

  private readonly apiUrl = environment.apiUrl;
  private readonly base = `${this.apiUrl}/api/dentist_contracts`;

  private authHeaders(): HttpHeaders {
    const token = this.loginService.token?.() ?? '';
    return new HttpHeaders({ Authorization: `Bearer ${token}` });
  }

  private handleError(err: unknown) {
    return throwError(() => err);
  }

  // CHANGE: contracts picker needs a list
  getAll(): Observable<DentistContractRow[]> {
    return this.http
      .get<DentistContractRow[]>(this.base, { headers: this.authHeaders() })
      .pipe(catchError((e) => this.handleError(e)));
  }

  // CHANGE: selection needs full details (contract + rates)
  getById(id: number): Observable<DentistContractWithRates> {
    return this.http
      .get<DentistContractWithRates>(`${this.base}/${id}`, { headers: this.authHeaders() })
      .pipe(catchError((e) => this.handleError(e)));
  }

  create(payload: CreateDentistContractRequest): Observable<DentistContractWithRates> {
    return this.http
      .post<DentistContractWithRates>(`${this.base}/`, payload, { headers: this.authHeaders() })
      .pipe(catchError((e) => this.handleError(e)));
  }

  patch(id: number, payload: PatchDentistContractRequest): Observable<DentistContractWithRates> {
    return this.http
      .patch<DentistContractWithRates>(`${this.base}/${id}`, payload, { headers: this.authHeaders() })
      .pipe(catchError((e) => this.handleError(e)));
  }
}
