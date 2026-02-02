// src/app/api_services/region-service.ts
import { Injectable, inject } from '@angular/core';
import {HttpClient, HttpHeaders, HttpParams} from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { LoginService } from '../login.service';

export interface PageResponse<T> {
  page: number;
  page_size: number;
  total_items: number;
  total_pages: number;
  items: T[];
}

export interface RegionRow {
  id: number;
  name: string;
}

export interface CreateRegionRequest {
  name: string;
}

export interface PatchRegionRequest {
  name?: string | null;
}

@Injectable({ providedIn: 'root' })
export class RegionService {
  private readonly http = inject(HttpClient);
  private readonly loginService = inject(LoginService);

  // Adjust if your backend base path differs.
  // Example: environment.apiBaseUrl = 'http://localhost:3000/api'
  private readonly baseUrl = `${environment.apiUrl}/api/regions`;

  private authHeaders(): HttpHeaders {
    const token = this.loginService.token?.() ?? '';
    return new HttpHeaders({ Authorization: `Bearer ${token}` });
  }

  getRegions(opts?: { page?: number; pageSize?: number }): Observable<PageResponse<RegionRow>> {
    let params = new HttpParams();
    if (opts?.page != null) params = params.set('page', String(opts.page));
    if (opts?.pageSize != null) params = params.set('page_size', String(opts.pageSize));

    return this.http.get<PageResponse<RegionRow>>(this.baseUrl,  { headers: this.authHeaders() });
  }

  getRegionById(id: number): Observable<RegionRow> {
    return this.http.get<RegionRow>(`${this.baseUrl}/${id}`,  { headers: this.authHeaders() });
  }

  createRegion(payload: CreateRegionRequest): Observable<RegionRow> {
    return this.http.post<RegionRow>(this.baseUrl, payload,  { headers: this.authHeaders() });
  }

  patchRegion(id: number, payload: PatchRegionRequest): Observable<RegionRow> {
    // If you prefer JSON Patch semantics, keep PATCH; your handler expects Json<PatchRegionRequest>.
    return this.http.patch<RegionRow>(`${this.baseUrl}/${id}`, payload);
  }
}
